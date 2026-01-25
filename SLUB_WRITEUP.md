# Le SLUB Allocator Linux

## 1. Introduction - Pourquoi ?

### Le problème
Imaginons qu'un noyau linux doit allouer de la mémoire pour les structures comme : 
- Descripteurs de fichiers
- Structures de processus
- Buffers réseau

En utilisant malloc classiquen on rencontrerais deux problèmes :
1. La lenteur car initiliser une structure a chaque fois peut être long
2. La fragmentation, la mémoire comporte plein de petits trous

### La solution
Au lieu de créer & détruire les objets, on les réutilise

#### Le principe
- On crée un cache pour chaque type d'objet
- Chaque cache va contenir des blocs de mémoire
- Chaque slab va contenir plusieurs objets qui seront pré-initialisés
- Et quand on libère un objet, il restera initialisé et pourra donc être réutilisé

### Pourquoi le slub en choix

Linux possède trois implémentations : 
- Slab : Original, complexe qui est maintenant obsolète
- Slub : Moderne, plus simple et c'est celui par défaut depuis linux 2.6.23
- Slob : Minimal, pour les systèmes embarqués

On se concentre donc sur le slub car c'est celui utilisé aujourd'hui.

--- 

## 2. L'architecture du SLUB

### Vue d'ensemble

Le Kernel contient des kmem_cache (qui sont des cache par type d'objet).

Chaque cache gère plusieurs slabs et chaque slab contient plusieurs objets de même tailles.

Nous avons donc : 

Kernel
|- Kmem_cache
|- Slab full (pour tout les objets alloués)
|- Slab partial (pour certains objets libres)
|_ Slab free (pour tout les objets libres)

### La Structure d'un objet

Un objet en mémoire : 

**Une redzone (si le débug est activé)**
- Zone avec une valeur magique
- Si elle change, alors => buffer overflow détecté

**Les données**
- Les données réelles de l'objet
- Ce que le code utilise

**le FreePointer**
- Quand l'objet est libre, cette zone contient l'adresse du prochain objet libre
- C'est une liste chainée des objets libres
- C'est également la cible principale pour l'exploitation

**Padding**
- Octets de remplissage pour l'alignement

### CPU Caches

Pour être rapide, SLUB garde un cache par CPU, chaque CPU a son propre SLAB actif et sa freelist.

**Les avantages**
- Pas besoin de look (chaque CPU va travailler sur son cache)
- Allocation très rapide
- Meilleur utilisattion du cache

--- 

## 3. l'allocation et la libération

### Allocation (fastpath)

Quand on alloue un objet, ce qu'il va se passer :
1. Regarder la freelist du cache par cpu
2. Si un objet est disponible 
    - il va le retirer de la freelist
    - retourner le pointeur
3. Sinon cela va passer au slowpath

### Allocation (slowpath)

si le fastpath échoue, on cherche dans cet ordre : 
1. Liste des slabs partiels du CPU
2. Liste des slabs partiels du noeud
3. Créer un nouveau slab
4. si impossible -> Out of memory

### Libération

Quand on libère un objet : 
1. Trouver a quel slab appartient l'objet
2. Ajouter l'objet au début de la freelist
3. Si le slab était full -> le déplacer dans partial
4. Si le slab devient empty -> possibilité de le libérer

---

## 4. Exploiatation du SLUB

Comment un bug mémoire peut être exploité à cause de la façon dont slub gère les objets.

### Use-After-Free

un Use-After-Free, c'est quand on utilise un objet après l'avoir libéré 

Exemple de scénario : 
1. On alloue un objet A
2. On libere un objet A
3. Le code garde un pointeur vers A et l'utilise encore
4. Entre temps, A peut être réalloué avec des données qui sont controlées par un attaquant

### Double-Free

Un double-free, c'est quand le même objet est libéré deux fois.

Effet sur la freelist : 
- la freelist est une liste chainée qui via le freepointer dans les objets libres
- En double-free, on peut créer une boucle ou réinsérer deux fois le même objet.
- Ensuite, en allouant plusieurs fois, on peut parfois controler le freepointer et le faire pointer ou on veut en mémoire

### Slab Spraying

Le slab spraying consiste à : 
- Allouer plein d'objet d'un meme cache avec des données controlées.
- Le but est de remplir un stab avec nos données
- Couplé à un UAF, ça augmente les chances que l'objet libéré soit réutilisé avec des valeurs choisies par l'attanquant.

---

## 5. Mécanismes de défense

Le noyau a ajouté plusieurs proctections pour rendre ces attaques plus difficiles.

### Redzones et Poisoning 
- Redzones : zones autour de l'objet remplies avec un motif spécial
- Si elles sont modifiées, le noyau peut détecter un overflow


- Poisoning : Quand un objet est libéré, il est rempli avec une valeur connue.
- a la ré-allocation, si cette valeur a changé, ca peut détecter un UAF

### Freelist randomization
- au lieu d'avoir une freelist dans un ordre
