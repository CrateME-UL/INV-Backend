### roadmap:

stocker tout l'aggrégat dans l'inventaire ça me semble un peu much et ça ne fonctionne pas dans un système distribué.
il faudrait que tout soit sauvegardé à mesure, donc après qu'une requête est terminée.

aussi, si le système devient immense, il va être difficile voir impossible de sauvegarder toute la BD à nouveau.

cependant, pour faciliter les règles métiers, elles devraient être faites le plus possible dans le domaine.

pour subvenir à cela, on pourrait faire la validation dans le domaine pour les valeurs et les duplicats. mais la logique de stockage resterait dans l'implémentation du repository.

j'ai besoin de valider la taille des entrées, les duplicats, les valeurs des nombres.

je n'ai pas nécessairement de Number quand je suis dans la ressource, mais dans le domaine, j'en ai un pour sûr pour chaque objet. je pourrais stocker le UUID comme un attribut dans le BD, le stockage ne sera jamais un enjeux et avoir l'auto-increment quand même permet d'itérer sur cet objet uniquement. nôtre logique fonctionne avec des UUID, mais le repo gère comme il veut le stockage, ce qui a plus de sens que d'avoir un ou l'autre en raison du stockage infini et on peut même faire la différence maintenant entre 2 items.

bref, il devrait y avoir un repo séparé pour le stockage, la requête terminée, la mémoire est effacée, l'objet doit persister.
la validation se fait dans le domaine le plus possible.

il faudrait faire une factory pour créer l'inventaire dans le service, avec le contexte de la hashmap fourni par le repo. cela va nous permettre de faire la validation.

dans un monde avec beaucoup de requêtes à la bd, on va favoriser la lecture à l'écriture et le moins de requêtes possibles, voir mettre de bons indexes et mettre une bonne cache pour les requêtes fréquentes.
