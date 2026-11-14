# Traductions françaises de l'interface (langue de repli).
# Chaque clé correspond à une variante de `TextId` (src/i18n.rs) ; un test
# vérifie que fr.ftl et en.ftl déclarent exactement les mêmes clés.
# Typographie : espace insécable U+00A0 avant « : ; ? ! », jamais U+202F
# (absente des polices d'egui) ; points de suspension U+2026 « … » (présents
# dans les polices d'egui, qui s'en servent pour tronquer).

## Écran d'accueil

welcome-title = Bienvenue dans { $product }
welcome-subtitle = Clone libre de MS Project — ouvrir, modifier, enregistrer
welcome-open-demo = Ouvrir l'exemple : construction d'une maison
welcome-open-file = Ouvrir un fichier MSPDI (.xml)…
welcome-new-project = Nouveau projet
welcome-web-note = Le fichier reste dans votre navigateur ; rien n'est envoyé.
welcome-language = Langue
welcome-theme = Thème

## Menus

menu-file = Fichier
menu-open = Ouvrir un fichier MSPDI (.xml)…
menu-open-demo = Ouvrir l'exemple
menu-close = Fermer
menu-view = Affichage
menu-zoom = Zoom
menu-theme = Thème
menu-language = Langue
menu-diagnostics = Diagnostics

## Zoom

toolbar-fit = Ajuster
zoom-day = Jour
zoom-week = Semaine
zoom-month = Mois
zoom-quarter = Trimestre

## Thème et langue (noms de langue dans leur propre langue)

theme-system = Système
theme-light = Clair
theme-dark = Sombre
lang-system = Système
lang-fr = Français
lang-en = English

## Colonnes de la table

col-id = N°
col-name = Nom de la tâche
col-duration = Durée
col-start = Début
col-finish = Fin
col-predecessors = Prédécesseurs
col-total-slack = Marge totale
col-critical = Critique
col-percent-complete = % achevé
col-resource-names = Noms ressources
col-baseline-start = Début de référence
col-baseline-finish = Fin de référence
col-finish-variance = Variation de fin
yes = Oui
no = Non

## Unités de durée (forme longue, hypothèse H-UI-04 : « 5 jours », « 1 jour? »)

unit-minute-one = minute
unit-minute-other = minutes
unit-hour-one = heure
unit-hour-other = heures
unit-day-one = jour
unit-day-other = jours
unit-week-one = semaine
unit-week-other = semaines
unit-month-one = mois
unit-month-other = mois
unit-minute-elapsed-one = minute écoulée
unit-minute-elapsed-other = minutes écoulées
unit-hour-elapsed-one = heure écoulée
unit-hour-elapsed-other = heures écoulées
unit-day-elapsed-one = jour écoulé
unit-day-elapsed-other = jours écoulés
unit-week-elapsed-one = semaine écoulée
unit-week-elapsed-other = semaines écoulées
unit-month-elapsed-one = mois écoulé
unit-month-elapsed-other = mois écoulés

## Types de liens (codes de MS Project en français)

link-fs = FD
link-ss = DD
link-ff = FF
link-sf = DF

## Conventions de formatage (lues par Texts::format_labels)

fmt-decimal-sep = ,
fmt-list-sep = ;
fmt-date-order = dmy
fmt-unknown = ?
# Règle de pluriel des unités : below-two (singulier si |v| < 2) ou
# exactly-one (singulier si |v| = 1).
fmt-plural-rule = below-two

## Mois

month-1-long = janvier
month-1-short = janv.
month-2-long = février
month-2-short = févr.
month-3-long = mars
month-3-short = mars
month-4-long = avril
month-4-short = avr.
month-5-long = mai
month-5-short = mai
month-6-long = juin
month-6-short = juin
month-7-long = juillet
month-7-short = juil.
month-8-long = août
month-8-short = août
month-9-long = septembre
month-9-short = sept.
month-10-long = octobre
month-10-short = oct.
month-11-long = novembre
month-11-short = nov.
month-12-long = décembre
month-12-short = déc.

## Jours de la semaine

weekday-mon-short = lun
weekday-mon-initial = L
weekday-tue-short = mar
weekday-tue-initial = M
weekday-wed-short = mer
weekday-wed-initial = M
weekday-thu-short = jeu
weekday-thu-initial = J
weekday-fri-short = ven
weekday-fri-initial = V
weekday-sat-short = sam
weekday-sat-initial = S
weekday-sun-short = dim
weekday-sun-initial = D

## Échelle de temps

tier-quarter = T
tier-week = S

## Barre d'état

status-tasks =
    { $count ->
        [one] { $count } tâche
       *[other] { $count } tâches
    }
status-range = { $start } – { $finish }
status-errors =
    { $count ->
        [0] aucune erreur
        [one] { $count } erreur
       *[other] { $count } erreurs
    }
status-warnings =
    { $count ->
        [0] aucun avertissement
        [one] { $count } avertissement
       *[other] { $count } avertissements
    }
status-consistent = Planning cohérent
status-no-document = Aucun document ouvert

## Ouverture de fichiers

open-error-title = Impossible d'ouvrir le fichier
open-error-read = Lecture du fichier impossible : { $detail }
open-error-mspdi = Fichier MSPDI illisible : { $detail }

## Nouveau, Enregistrer, Enregistrer sous (lot L13)

menu-new = Nouveau
menu-save = Enregistrer
menu-save-as = Enregistrer sous…
menu-quit = Quitter
new-project-name = Projet1
dialog-save-title = Enregistrer le projet (MSPDI)
save-in-progress = Un enregistrement est déjà en cours.
save-error-title = Impossible d'enregistrer le fichier
save-error-write = Écriture du fichier impossible : { $detail }
save-adjusted =
    { $count ->
        [one] Enregistré : { $count } valeur ajustée au format MSPDI.
       *[other] Enregistré : { $count } valeurs ajustées au format MSPDI.
    }
confirm-unsaved-title = Enregistrer les modifications de « { $name } » ?
confirm-save = Enregistrer
confirm-discard = Ne pas enregistrer
confirm-cancel = Annuler

## Bandeau de messages

message-dismiss = Fermer le message
range-truncated = Plage d'affichage limitée à 20 ans

## Accessibilité et divers

row-label = Ligne { $id } : { $name }
splitter-label = Séparateur entre la table et le Gantt
gantt-scroll-label = Défilement horizontal du Gantt
demo-file-name = Exemple : construction d'une maison
file-filter-mspdi = Fichier MSPDI (XML)
dialog-open-title = Ouvrir un fichier MSPDI
state-load-warning = Préférences d'affichage ignorées : { $detail }

## Menu Édition

menu-edit = Édition
menu-undo = Annuler
menu-redo = Rétablir

## Structure (menu contextuel, confirmation de suppression)

structure-insert = Insérer une tâche
structure-delete = Supprimer la tâche
structure-indent = Abaisser
structure-outdent = Hausser
structure-link = Lier les tâches
structure-unlink = Supprimer les liaisons
confirm-delete-title = Supprimer une tâche récapitulative
confirm-delete-text =
    { $count ->
        [one] La tâche sera supprimée.
       *[other] La récapitulative et ses sous-tâches seront supprimées ({ $count } tâches).
    }
confirm-delete-yes = Supprimer
confirm-delete-no = Annuler

## Refus de commande (une clé par code d'erreur du moteur, `error-<code>`)

command-error-title = Modification refusée
error-unknown = Erreur : { $detail }
error-assignment-duplicate = Ressource déjà affectée : { $detail }
error-assignment-kind_mismatch = Champ contraire au type de la ressource : { $detail }
error-assignment-not_found = Affectation introuvable : { $detail }
error-assignment-zero_units_fixed_work = Unités nulles sur une tâche à travail fixe : { $detail }
error-baseline-exists = Ligne de base déjà enregistrée : { $detail }
error-baseline-rollup_needs_selection = Cumul sans sélection : { $detail }
error-batch-duplicate_label = Étiquette en double : { $detail }
error-batch-unknown_label = Étiquette inconnue : { $detail }
error-batch-unsupported_version = Version de format non prise en charge : { $detail }
error-batch-would_create_errors = Modification qui créerait des erreurs : { $detail }
error-calendar-exception_not_found = Exception de calendrier introuvable : { $detail }
error-calendar-exception_overlap = Exceptions de calendrier qui se chevauchent : { $detail }
error-calendar-not_found = Calendrier introuvable : { $detail }
error-command-not_yet_available = Commande pas encore disponible : { $detail }
error-explain-not_scheduled = Tâche non planifiée : { $detail }
error-file-invalid = Fichier illisible : { $detail }
error-file-open_in_app = Fichier ouvert dans l'application : { $detail }
error-file-outside_root = Chemin non autorisé : { $detail }
error-file-read = Lecture impossible : { $detail }
error-file-too_large = Fichier trop volumineux : { $detail }
error-file-write = Écriture impossible : { $detail }
error-history-empty = Rien à annuler ni à rétablir : { $detail }
error-history-inconsistent = Historique incohérent : { $detail }
error-limit-exceeded = Limite dépassée : { $detail }
error-link-duplicate = Lien déjà présent : { $detail }
error-link-not_found = Lien introuvable : { $detail }
error-link-self = Lien d'une tâche vers elle-même : { $detail }
error-link-summary_descendant = Lien entre une récapitulative et sa sous-tâche : { $detail }
error-param-conflict = Valeurs qui s'excluent : { $detail }
error-param-invalid = Valeur refusée : { $detail }
error-param-mismatch = Valeurs contradictoires : { $detail }
error-progress-duration_below_actual = Durée sous la durée réelle : { $detail }
error-progress-inactive = Tâche inactive : { $detail }
error-progress-invalid_order = Fin réelle avant le début réel : { $detail }
error-progress-summary_not_editable = Avancement d'une récapitulative : { $detail }
error-resource-has_assignments = Ressource affectée : { $detail }
error-resource-not_found = Ressource introuvable : { $detail }
error-session-none = Aucun document : { $detail }
error-session-poisoned = Session indisponible : { $detail }
error-task-computed_field = Champ calculé : { $detail }
error-task-fixed_work_effort_driven = Travail fixe toujours piloté par l'effort : { $detail }
error-task-has_children = Récapitulative avec sous-tâches : { $detail }
error-task-manual_unsupported = Tâche planifiée manuellement : { $detail }
error-task-move_into_subtree = Déplacement impossible : { $detail }
error-task-not_found = Tâche introuvable : { $detail }
error-task-outline_refused = Hiérarchie refusée : { $detail }
error-text-bad_date = Date illisible : { $detail }
error-text-bad_link_type = Type de lien inconnu : { $detail }
error-text-bad_number = Nombre illisible : { $detail }
error-text-empty = Saisie vide : { $detail }
error-text-external_predecessor = Prédécesseur externe : { $detail }
error-text-negative = Valeur négative : { $detail }
error-text-out_of_range = Valeur hors limites : { $detail }
error-text-too_long = Saisie trop longue : { $detail }
error-text-unknown_unit = Unité inconnue : { $detail }

## Édition des cellules (lot L12)

blank-row-label = Ligne vide (nouvelle tâche)
cell-edit-label = Saisie : { $column }
edit-error-unknown-id = Aucune tâche d'ID { $id }
edit-error-not-editable = Colonne calculée, non modifiable

## J7 — L0 : Codes de refus du contrat (ADR-0015)

error-calendar-name_taken = Nom de calendrier déjà pris : { $detail }
error-calendar-base_not_base = Base de calendrier invalide : { $detail }
error-calendar-in_use = Calendrier utilisé : { $detail }
error-gesture-not_draggable = Geste impossible sur cette barre : { $detail }
error-usage-limit = Rapport d'utilisation trop grand : { $detail }
error-control-rejected = Refusé dans l'application : { $detail }
error-control-approval_timeout = Aucune réponse dans le délai : { $detail }
error-control-not_relayable = Commande non relayable : { $detail }
error-control-not_attached = Aucune application rattachée : { $detail }
error-control-app_gone = Application fermée : { $detail }
error-control-screenshot_unavailable = Capture indisponible : { $detail }
error-control-screenshot_too_large = Capture trop grande : { $detail }
error-control-busy = Application occupée : { $detail }
error-control-document_changed = Document de l'application changé : { $detail }

# --- J7 L1 ---
table-label = Table des tâches
nav-cursor-label = Ligne { $id }, { $column }, { $value }
nav-cursor-blank-label = Ligne vide, { $column }
nav-cursor-empty-value = vide

# --- J7 L2 ---

# --- J7 L3 ---
approval-title = L'agent propose sur « { $document } » : { $name }
approval-changed =
    { $touched ->
        [0] aucune tâche modifiée
        [one] { $touched } tâche modifiée
       *[other] { $touched } tâches modifiées
    } · { $moved ->
        [0] aucune date déplacée
        [one] { $moved } date déplacée
       *[other] { $moved } dates déplacées
    }
approval-finish = fin { $value }
approval-finish-same = fin inchangée
approval-critical = chemin critique modifié
approval-warnings =
    { $count ->
        [one] { $count } avertissement
       *[other] { $count } avertissements
    }
approval-queued =
    { $count ->
        [one] { $count } autre proposition en attente
       *[other] { $count } autres propositions en attente
    }
approval-accept = Accepter
approval-reject = Refuser
menu-undo-agent = Annuler : agent — { $detail }
menu-undo-agent-ops =
    { $count ->
        [one] { $count } opération
       *[other] { $count } opérations
    }

# --- J7 L4 ---

# --- J7 L5 ---

# --- J7 L6 ---
fmt-percent-suffix = {"\u00A0%"}
column-header-label = Colonne { $column }
column-resize-label = Largeur de la colonne { $column }
column-menu-hide = Masquer la colonne
table-scroll-label = Défilement horizontal de la table

# --- J7 L7 ---
menu-gantt-standard = Diagramme de Gantt
menu-gantt-tracking = Gantt de suivi

# --- J7 L8 ---
gantt-reason-summary = récapitulative (seul le lien est permis)
gantt-reason-started = tâche commencée
gantt-reason-milestone = jalon
gantt-reason-excluded = tâche exclue du calcul
gantt-reason-blank = ligne vide
gantt-reason-overflow = écart hors des dates permises
gantt-reason-same-task = lien d'une tâche vers elle-même
link-dialog-title = Dépendance
link-dialog-from = De
link-dialog-to = À
link-dialog-type = Type
link-dialog-lag = Décalage
link-dialog-delete = Supprimer
link-dialog-stale = Le projet a changé : la dépendance a été relue.
link-dialog-task = { $id } { $name }

# --- J7 L9 ---
task-info-menu = Informations sur la tâche…
task-info-title = Informations sur la tâche — { $id } { $name }
task-info-tab-general = Général
task-info-tab-predecessors = Prédécesseurs
task-info-tab-resources = Ressources
task-info-tab-advanced = Avancé
task-info-tab-notes = Notes
task-info-name = Nom
task-info-duration = Durée
task-info-percent = % achevé
task-info-start = Début
task-info-finish = Fin
task-info-active = Active
task-info-pred-id = ID
task-info-pred-type = Type
task-info-pred-lag = Décalage
task-info-row-field = { $column }, ligne { $count }
task-info-add-predecessor = Ajouter un prédécesseur
task-info-add-resource = Affecter une ressource
task-info-remove-row = Retirer
task-info-resource-name = Ressource
task-info-resource-units = Unités
task-info-no-resource = Aucune ressource dans le projet.
task-info-constraint = Type de contrainte
task-info-constraint-date = Date de contrainte
task-info-deadline = Échéance
task-info-task-type = Type de tâche
task-info-effort-driven = Pilotée par l'effort
task-info-notes = Notes de la tâche
task-info-link-fs = Fin à début (FD)
task-info-link-ss = Début à début (DD)
task-info-link-ff = Fin à fin (FF)
task-info-link-sf = Début à fin (DF)
task-info-constraint-asap = Dès que possible
task-info-constraint-alap = Le plus tard possible
task-info-constraint-mso = Doit commencer le
task-info-constraint-mfo = Doit finir le
task-info-constraint-snet = Début au plus tôt le
task-info-constraint-snlt = Début au plus tard le
task-info-constraint-fnet = Fin au plus tôt le
task-info-constraint-fnlt = Fin au plus tard le
task-info-type-fixed-units = Capacité fixe
task-info-type-fixed-duration = Durée fixe
task-info-type-fixed-work = Travail fixe
task-info-ok = OK
task-info-cancel = Annuler
task-info-stale = Le projet a changé depuis l'ouverture : les valeurs ont été relues, rien n'a été appliqué.
task-info-date-required = Date obligatoire pour ce type de contrainte
task-info-bad-id = ID illisible : { $id }
task-info-duplicate-predecessor = Prédécesseur en double : { $id }
task-info-duplicate-resource = Ressource affectée deux fois : { $name }
task-info-unknown-resource = Ressource inconnue

# --- J7 L10 ---
menu-view-resource-sheet = Feuille des ressources
res-sheet-label = Feuille des ressources
res-col-indicator = Ind.
res-col-name = Nom de la ressource
res-col-kind = Type
res-col-material-label = Étiquette
res-col-initials = Initiales
res-col-group = Groupe
res-col-max-units = Capacité max.
res-col-standard-rate = Taux standard
res-col-overtime-rate = Taux heures sup.
res-col-cost-per-use = Coût/utilisation
res-col-calendar = Calendrier
res-kind-work = Travail
res-kind-material = Matériel
res-kind-cost = Coût
rate-minute = /min
rate-hour = /h
rate-day = /j
rate-week = /sem
rate-month = /mois
rate-year = /an
res-row-label = Ressource { $id } : { $name }
res-blank-row-label = Ligne vide (nouvelle ressource)
res-name-first = Saisir d'abord le nom de la ressource
res-unknown-calendar = Aucun calendrier nommé « { $name } »
res-calendar-none = (aucun)
res-delete-title = Supprimer une ressource affectée
res-delete-text =
    { $count ->
        [one] « { $name } » est affectée à une tâche : l'affectation sera retirée avec elle.
       *[other] « { $name } » a { $count } affectations : elles seront retirées avec elle.
    }

# --- J7 L11 ---
menu-view-resource-usage = Utilisation des ressources
usage-sheet-label = Utilisation des ressources
usage-periods-label = Travail par période
usage-col-expand = +/-
usage-col-name = Nom de la ressource
usage-col-work = Travail
usage-hours-suffix = {"\u00A0"}h
usage-expand = Développer { $name }
usage-collapse = Réduire { $name }
usage-assignment-label = Affectation : { $name } ({ $detail })
usage-limit = Utilisation trop grande pour être affichée : replier des ressources ({ $detail }).

# --- J7 L12 ---
menu-project = Projet
menu-project-info = Informations sur le projet…
menu-working-time = Modifier le temps de travail…
menu-baseline = Ligne de base…
menu-reschedule = Replanifier le travail restant…
scope-project = Projet entier
scope-selection = { $count ->
        [one] Tâche sélectionnée (1)
       *[other] Tâches sélectionnées ({ $count })
    }
wt-title = Modifier le temps de travail
wt-calendar = Pour le calendrier
wt-project-calendar = { $name } (calendrier du projet)
wt-resource-calendar = { $name } (ressource)
wt-new = Nouveau calendrier…
wt-delete = Supprimer le calendrier
wt-name = Nom du calendrier
wt-base-of = Calendrier de base : { $name }
wt-new-name = Nom du nouveau calendrier
wt-new-base = Nouveau calendrier de base
wt-new-copy = Copie de « { $name } »
wt-create = Créer
wt-new-cancel = Ne pas créer
wt-prev-month = Mois précédent
wt-next-month = Mois suivant
wt-month-title = { $name } { $value }
wt-month-unavailable = Calendrier indisponible : { $detail }
wt-state-working = Ouvré : { $value }
wt-state-non-working = Chômé
wt-legend-working = Ouvré
wt-legend-non-working = Chômé
wt-legend-exception = Exception
wt-tab-exceptions = Exceptions
wt-tab-week = Semaine de travail
wt-exc-name = Nom
wt-exc-from = Début
wt-exc-to = Fin
wt-exc-working = Ouvré
wt-exc-ranges = Plages
wt-exc-recurring = Récurrente (lecture seule)
wt-exc-add = Ajouter une exception
wt-week-inherit = Hérité de la base
wt-week-non-working = Jour chômé
wt-week-working = Horaires définis
wt-week-setting = Réglage du { $name }
wt-week-ranges = Plages du { $name }
wt-week-inherited = Base : { $value }
wt-bad-ranges = Plages attendues sous la forme 08:00-12:00, 13:00-17:00
pi-title = Informations sur le projet
pi-direction = Planifier à partir de
pi-from-start = la date de début du projet
pi-from-finish = la date de fin du projet
pi-start = Date de début
pi-finish = Date de fin
pi-status = Date d'état
pi-status-hint = Vide : NA (la date du jour sert de date d'état)
pi-calendar = Calendrier
bl-title = Ligne de base
bl-set = Définir la ligne de base
bl-clear = Effacer la ligne de base
bl-number = Numéro
bl-baseline-0 = Ligne de base
bl-baseline-n = Ligne de base { $count }
bl-rollup = Cumul des lignes de base
bl-rollup-none = Aucun cumul
bl-rollup-all = Vers toutes les tâches récapitulatives
bl-rollup-from = Des sous-tâches vers les récapitulatives sélectionnées
bl-confirm = { $detail } Remplacer la ligne de base enregistrée ?
bl-replace = Remplacer
rs-title = Replanifier le travail restant
rs-after = Replanifier après le

# --- J7 L13 ---
menu-assign-resources = Affecter des ressources…
as-title = Affecter des ressources
as-task-one = Tâche : { $name }
as-task-many = { $count ->
        [one] { $count } tâche sélectionnée
       *[other] { $count } tâches sélectionnées
    }
as-no-task = Aucune tâche sélectionnée
as-filter = Filtrer par nom
as-resource = Nom de la ressource
as-units = Unités
as-units-of = Unités de { $name }
as-assigned = Affectée
as-assigned-all = oui
as-assigned-some = { $count } sur { $value }
as-assign = Affecter
as-remove = Retirer
as-update-units = Modifier les unités
as-close = Fermer
as-no-resource = Aucune ressource
as-hint = Clic sur une ressource pour la choisir (Ctrl ou Cmd + clic : plusieurs) ; unités laissées vides : valeur par défaut.

# --- J7 L14 ---
open-error-read-file = Lecture de « { $name } » impossible : { $detail }
open-error-mspdi-file = « { $name } » : fichier MSPDI illisible : { $detail }
open-error-missing = « { $name } » est introuvable, retiré des fichiers récents : { $detail }
menu-recent = Fichiers récents
menu-recent-empty = Aucun fichier récent
menu-recent-missing = { $name } (introuvable)

# --- J7 L15 ---
recovery-title = Récupérer « { $name } » ({ $count ->
        [one] { $count } modification
       *[other] { $count } modifications
    }) ?
recovery-checkpointed = Les modifications antérieures au dernier point de reprise ne pourront plus être annulées.
recovery-restore = Restaurer
recovery-discard = Ignorer
recovery-later = Plus tard
recovery-partial = Récupération partielle de « { $name } » : { $count ->
        [one] { $count } modification rejouée
       *[other] { $count } modifications rejouées
    }, la suivante est refusée : { $detail }
recovery-failed = Récupération de « { $name } » impossible : { $detail }
recovery-discard-failed = Suppression du journal de reprise impossible : { $detail }
recovery-journal-cut = Reprise après plantage interrompue pour ce document : { $detail }. L'édition continue ; enregistrez régulièrement.
recovery-file-changed = Le fichier a changé sur le disque depuis : la version récupérée s'ouvrira sans chemin (Enregistrer sous), pour ne pas l'écraser.
recovery-kept = Journal d'une autre version de PlannerCraft, ou trop grand : il ne se restaure pas ici. Ignorer le supprime.
recovered-file-name = { $stem } (récupéré)

# --- J7 L16 ---

# --- J8 L0 : codes de refus du contrat (ADR-0016) ---
error-paste-unrecognized = Texte collé non reconnu : { $detail }
error-paste-too_large = Texte collé trop grand : { $detail }
error-paste-links_dropped = Liens non recopiés : { $detail }
error-style-ignored = Mise en forme du fichier ignorée : { $detail }
error-style-invalid = Mise en forme invalide : { $detail }
error-pdf-too_many_pages = PDF trop long : { $detail }
error-pdf-glyph_missing = Caractère absent des polices du PDF : { $detail }
error-pdf-logo_invalid = Logo illisible : { $detail }
error-task-manual_finish_before_start = Fin avant le début de la tâche manuelle : { $detail }
error-control-bad_endpoint = Adresse du canal de contrôle invalide : { $detail }
error-custom-unknown_field = Champ personnalisé inconnu : { $detail }
error-custom-value_type = Valeur de champ personnalisé refusée : { $detail }
error-custom-too_long = Texte trop long : { $detail }
error-custom-read_only = Champ défini par le fichier : { $detail }

# --- J8 L1 ---

# --- J8 L2 ---

# --- J8 L3 ---

# --- J8 L4 ---

# --- J8 L5 ---

# --- J8 L6 ---

# --- J8 L7 ---

# --- J8 L8 ---

# --- J8 L9 ---
menu-filter = Filtre
filter-all = Toutes les tâches
filter-critical = Tâches critiques
filter-late = Tâches en retard
filter-incomplete = Tâches inachevées
filter-milestones = Jalons
filter-summaries = Tâches récapitulatives
filter-using-resource = Utilisant la ressource…
filter-date-range = Plage de dates…
filter-custom = Filtre personnalisé…
filter-reapply = Réappliquer le filtre
filter-clear = Retirer le filtre
filter-using-named = Utilisant { $name }
status-filter = Filtre : { $name }
filter-custom-title = Filtre personnalisé
filter-resource-title = Utilisant la ressource
filter-date-range-title = Plage de dates
filter-apply = Appliquer
filter-resource = Ressource
filter-no-resource = Aucune ressource dans le projet
filter-use-resource = Affectées à la ressource
filter-from = Du
filter-to = Au
filter-active-only = Tâches actives
filter-outline-max = Niveau au plus
filter-name-contains = Nom contenant
filter-related = Afficher les récapitulatives liées
filter-date-order = Le premier jour suit le dernier.
filter-dates-required = Indiquez le premier et le dernier jour.
filter-resource-missing = Choisissez une ressource.
menu-sort = Trier…
menu-sort-by-id = Trier par ID
sort-title = Trier
sort-by = Trier par
sort-ascending = Croissant
sort-descending = Décroissant
sort-keep-outline = Conserver la structure
sort-apply = Trier les tâches
sort-reset = Réinitialiser (ordre des ID)
menu-outline-level = Niveau hiérarchique
outline-level-n = Niveau { $count }
outline-all = Tous les niveaux
menu-hide-subtasks = Masquer les sous-tâches
menu-show-subtasks = Afficher les sous-tâches
outline-expand-row = Déplier { $name }
outline-collapse-row = Replier { $name }
view-reorder-locked = Retirez le tri ou le filtre pour déplacer, hausser ou abaisser des tâches.
menu-find = Rechercher…
search-label = Rechercher
search-previous = Précédent
search-next = Suivant
search-all-columns = Colonnes visibles
search-close = Fermer la recherche
search-none = Aucun résultat
search-counter = { $id }/{ $count }
search-hidden =
    { $count ->
        [one] ({ $count } masquée par le filtre)
       *[other] ({ $count } masquées par le filtre)
    }

# --- J8 L10 ---
menu-cut = Couper
menu-copy = Copier
menu-paste = Coller
menu-paste-special = Collage spécial…
menu-import-csv = Importer CSV…
paste-report-title = Collage
paste-report-tasks = { $count ->
        [one] { $count } tâche collée
       *[other] { $count } tâches collées
    }
paste-report-links = { $count ->
        [one] { $count } lien vers une tâche non copiée non repris
       *[other] { $count } liens vers des tâches non copiées non repris
    }
paste-report-assignments = { $count ->
        [one] { $count } affectation omise (ressource absente)
       *[other] { $count } affectations omises (ressources absentes)
    }
paste-report-resources = { $count ->
        [one] { $count } ressource créée
       *[other] { $count } ressources créées
    }
paste-hint-unrecognized = collez une ligne d'en-tête (Nom, Durée…) suivie des lignes, ou une seule colonne de noms
paste-hint-too-large = 10 000 lignes et 4 Mio au plus
paste-use-shortcut = Le navigateur ne donne pas le presse-papiers à un menu : collez avec Ctrl+V (Cmd+V sur Mac).
ps-title = Collage spécial
ps-source = { $count ->
        [one] Dernière copie : { $count } tâche
       *[other] Dernière copie : { $count } tâches
    }
ps-links = Recréer les liens entre les tâches collées
ps-resources = Affectations
ps-res-match = Par nom de ressource (omises si la ressource n'existe pas)
ps-res-create = Par nom, en créant les ressources absentes
ps-res-drop = Ne pas coller les affectations
ps-nothing = Rien à coller : copiez d'abord des lignes (Ctrl+C).
ci-pick-title = Importer un fichier CSV
ci-pick-filter = Fichiers CSV
ci-title = Importer CSV
ci-file = Fichier : { $name }
ci-separator = Séparateur
ci-encoding = Encodage
ci-language = Langue
ci-auto = Automatique
ci-sep-semicolon = Point-virgule (;)
ci-sep-comma = Virgule (,)
ci-sep-tab = Tabulation
ci-enc-utf8 = UTF-8
ci-enc-utf16le = UTF-16 LE
ci-enc-utf16be = UTF-16 BE
ci-enc-windows1252 = Windows-1252
ci-detected = Retenu : { $detail }
ci-columns = Colonnes
ci-rows = { $count ->
        [one] Aperçu ({ $count } ligne)
       *[other] Aperçu ({ $count } lignes)
    }
ci-recognized = reconnue
ci-unknown = ignorée (inconnue)
ci-unsupported = non prise en charge
ci-duplicate = ignorée (en double)
ci-diagnostics = { $count ->
        [one] { $count } remarque sur les lignes montrées
       *[other] { $count } remarques sur les lignes montrées
    }
ci-diagnostic-line = Ligne { $count } : { $detail }
ci-import = Importer
ci-imported = { $count ->
        [one] « { $name } » importé avec { $count } remarque (valeur par défaut ou colonne ignorée).
       *[other] « { $name } » importé avec { $count } remarques (valeurs par défaut ou colonnes ignorées).
    }
error-csv-error-too_large = Fichier CSV trop grand (64 Mio au plus).
error-csv-error-empty = Fichier CSV vide.
error-csv-error-no_header = Aucune ligne d'en-tête dans le fichier.
error-csv-error-no_known_column = Aucune colonne Nom ni Durée dans l'en-tête.
error-csv-error-too_many_rows = Trop de lignes (200 000 au plus).
error-csv-error-too_many_columns = Trop de colonnes (256 au plus).
error-csv-error-encoding = Le fichier n'est pas dans l'encodage indiqué.
error-csv-error-not_yet_available = Fonction pas encore disponible.

# --- J8 L11 ---
col-mode = Mode
fmt-mode-manual = Manuelle
fmt-mode-auto = Automatique
gantt-status-label = État { $date }

# --- J8 L12 ---
menu-ai-panel = Opérations IA
side-tab-ai = Opérations IA
side-tab-why = Pourquoi cette date ?
side-collapse = Réduire le volet
ai-no-pending = Aucune proposition en attente.
ai-more =
    { $count ->
        [one] { $count } autre tâche modifiée
       *[other] { $count } autres tâches modifiées
    }
ai-critical-entered = devient critique
ai-critical-left = n'est plus critique
ai-added = + { $name }
ai-removed = supprimée : { $name }
ai-no-preview = Pas d'aperçu pour cette opération : { $detail }
ai-history = Historique de la session
ai-history-empty = Aucune proposition traitée pour l'instant.
ai-history-line = { $value } — { $name } sur « { $document } » : { $detail }
ai-outcome-accepted = acceptée
ai-outcome-rejected = refusée
ai-outcome-expired = expirée
ai-outcome-stale = périmée (document fermé ou remplacé)
ai-outcome-failed = échouée ({ $detail })
ai-undo-entry = Annuler cette opération
why-no-task = Sélectionnez une tâche pour voir ce qui fixe ses dates.
why-drivers = Pilotes (cliquer pour sélectionner)
why-unavailable = Explication indisponible : { $detail }

# --- J8 L13 ---
web-start-failed = { $product } ne peut pas démarrer dans ce navigateur (WebGL2 ou WebGPU indisponible ?). Essayez un navigateur récent ou la version de bureau. Détail : { $detail }
dropped-first-only = { $count } fichiers déposés : seul « { $name } » est ouvert.
menu-export = Exporter
menu-export-pdf = PDF…
menu-export-svg = SVG…
menu-export-csv = CSV des tâches…
export-done = Exporté : { $name }
export-error-title = Export impossible
export-error-failed = L'export a échoué : { $detail }

# --- J8 L14 ---
diag-calendar-unknown = calendrier { $calendar } inconnu, référencé par { $referenced_by }
diag-calendar-base_missing = le calendrier { $calendar } hérite du calendrier { $base }, qui n'existe pas
diag-calendar-base_not_base = le calendrier { $calendar } hérite du calendrier { $base }, qui n'est pas un calendrier de base
diag-calendar-inheritance_cycle = boucle d'héritage entre calendriers : { $chain }
diag-calendar-no_working_time = le calendrier { $calendar } n'a aucun temps ouvré : aucune durée ne peut y être placée
diag-calendar-internal = le calendrier { $calendar } n'a pas pu être compilé (incohérence interne, anomalie à signaler)
diag-calendar-exceptions_overlap = calendrier { $calendar } : les exceptions n° { $kept } et n° { $ignored } se chevauchent ; la n° { $kept } l'emporte sur les jours communs
diag-calendar-work_weeks_overlap = calendrier { $calendar } : les semaines de travail n° { $kept } et n° { $ignored } se chevauchent ; la n° { $kept } l'emporte sur les jours communs
diag-calendar-recurrence_unsupported = calendrier { $calendar } : la récurrence de l'exception n° { $exception } n'est pas prise en charge ; l'exception est ignorée
diag-project-calendar_not_base = le calendrier du projet ({ $calendar }) n'est pas un calendrier de base
diag-task-reserved_uid = une tâche porte l'UID 0, réservé à la tâche récapitulative du projet
diag-task-outline_jump = la tâche { $task } saute un niveau hiérarchique (ou le premier niveau n'est pas 1)
diag-task-null_with_data = la ligne vide { $task } porte des liens ou des affectations, ignorés
diag-task-negative_progress = la tâche { $task } a une durée réelle ou restante négative
diag-link-self = la tâche { $task } est liée à elle-même
diag-link-duplicate = plusieurs liens de la tâche { $predecessor } vers la tâche { $successor }
diag-link-unknown_task = la tâche { $successor } est liée à la tâche { $predecessor }, qui n'existe pas
diag-assignment-unknown_task = l'affectation { $assignment } porte sur la tâche { $task }, qui n'existe pas
diag-assignment-unknown_resource = l'affectation { $assignment } porte sur la ressource { $resource }, qui n'existe pas
diag-conversion-out_of_domain = calendrier { $calendar } : { $minutes } min à partir du { $from } sortent du domaine des dates (1984-2149)
diag-link-cycle = liens circulaires entre les tâches { $tasks }
diag-link-ignored_in_cycle = le lien { $predecessor } -> { $successor } est ignoré pour rompre un cycle
diag-link-summary_descendant = le lien { $predecessor } -> { $successor } relie une récapitulative à l'une de ses sous-tâches : ignoré
diag-link-finish_to_summary = le lien { $predecessor } -> { $successor } (fin à fin ou début à fin) vers une récapitulative s'applique à chacune de ses sous-tâches
diag-link-expansion_limit = { $ignored ->
        [one] { $ignored } lien fin à fin ou début à fin vers une récapitulative ignoré
       *[other] { $ignored } liens fin à fin ou début à fin vers une récapitulative ignorés
    } : leur report sur les sous-tâches dépasserait { $limit } arcs
diag-link-start_from_summary = le lien { $predecessor } -> { $successor } (début à début ou début à fin) depuis une récapitulative plafonne le début de chacune de ses sous-tâches (planification à rebours)
diag-link-inactive = le lien { $predecessor } -> { $successor } part d'une tâche inactive : sans effet
diag-link-excluded_endpoint = le lien { $predecessor } -> { $successor } touche une tâche non planifiée : ignoré
diag-link-manual_not_honored = la tâche manuelle { $successor } ne respecte pas son lien avec la tâche { $predecessor }
diag-link-unschedulable = le lien { $predecessor } -> { $successor } vers une récapitulative ne peut pas être appliqué ({ $cause }) : ignoré
diag-task-manual_undated = la tâche manuelle { $task } n'a ni début ni fin : non planifiée
diag-task-manual_dates_inverted = la tâche manuelle { $task } finit avant de commencer : fin ramenée au début
diag-task-calendar_fallback = le calendrier { $calendar } de la tâche { $task } est inutilisable : calendrier du projet utilisé
diag-project-calendar_fallback = le calendrier du projet ({ $calendar }) est inutilisable : calendrier Standard intégré utilisé
diag-task-unschedulable = la tâche { $task } ne peut pas être planifiée ({ $pass } : { $cause })
diag-constraint-override = la contrainte « { $kind } » de la tâche { $task } est respectée malgré { $bound }
diag-constraint-not_honored = la contrainte « { $kind } » du { $date } de la tâche { $task } n'est pas respectée : les liens l'emportent
diag-constraint-summary_ignored = la contrainte « { $kind } » de la récapitulative { $task } est ignorée
diag-constraint-manual_ignored = la contrainte « { $kind } » de la tâche manuelle { $task } est ignorée par le calcul
diag-task-deadline_missed = la tâche { $task } finit le { $finish }, après son échéance du { $deadline }
diag-resource-overallocated = la ressource { $resource } est surutilisée du { $from } au { $to } (pic { $peak } %)
diag-resource-availability_ignored = les disponibilités datées de la ressource { $resource } sont ignorées : capacité constante
diag-assignment-contour_unsupported = le profil { $contour } de l'affectation { $assignment } est placé comme uniforme
diag-assignment-zero_units = l'affectation { $assignment } a des unités nulles : la durée de la tâche est gardée
diag-assignment-duration_out_of_domain = la durée de l'affectation { $assignment } sort du domaine
diag-assignment-calendar_empty = les calendriers { $task_calendar } (tâche) et { $resource_calendar } (ressource) de l'affectation { $assignment } n'ont aucun temps ouvré commun
diag-task-fixed_work_effort_driven = la tâche { $task } est à travail fixe sans être pilotée par l'effort
diag-calendar-too_many_segments = la combinaison des calendriers { $calendar } et { $other } dépasse { $limit } segments
diag-assignment-too_many_parts = la tâche { $task } a { $parts } affectations de travail (plus de { $limit }) : sa durée est celle de la plus longue
diag-progress-link_ignored = le lien { $predecessor } -> { $successor } est sans effet : la tâche en cours { $successor } ne se fractionne pas
diag-progress-link_overridden = les dates réelles de la tâche achevée { $successor } l'emportent sur son lien avec la tâche { $predecessor }
diag-progress-option_unsupported = l'option d'avancement { $option } n'est pas suivie : calcul aux valeurs par défaut
diag-progress-from_finish_in_progress = la tâche en cours { $task } d'un projet planifié à rebours : son reste est placé au plus tôt
diag-progress-actual_start_missing = la tâche commencée { $task } n'a pas de début réel : son début planifié est pris
diag-progress-milestone_rounded = le jalon { $task } est passé à 100 % achevé
diag-constraint-ignored_in_progress = la contrainte « { $kind } » de la tâche en cours { $task } est ignorée pour ses dates
diag-baseline-cost_not_computed = ligne de base { $number } : coûts recopiés tels quels, sans recalcul
diag-baseline-task_skipped = { $count ->
        [one] { $count } tâche non planifiée sans ligne de base
       *[other] { $count } tâches non planifiées sans ligne de base
    }
diag-baseline-nothing_to_clear = ligne de base { $number } : rien à effacer
diag-schedule-internal = incohérence interne du calcul (anomalie à signaler) : { $detail }
diag-task-custom_invalid = la tâche { $task } a une valeur de { $field } du mauvais type ou hors des bornes
diag-mspdi-invalid_value = valeur illisible ou hors bornes : valeur par défaut appliquée
diag-mspdi-unknown_code = code inconnu ({ $code }) : valeur par défaut appliquée
diag-mspdi-missing_required = élément obligatoire absent
diag-mspdi-duplicate = élément en double : le premier est gardé
diag-mspdi-dangling_reference = référence vers un UID inexistant
diag-mspdi-link_ignored = lien ignoré ({ $reason })
diag-mspdi-rounded = valeur arrondie à la minute
diag-mspdi-unsupported = fonction non prise en charge ({ $what })
diag-mspdi-inconsistent = incohérence tolérée
diag-mspdi-truncated = { $dropped ->
        [one] { $dropped } diagnostic supprimé au-delà du plafond
       *[other] { $dropped } diagnostics supprimés au-delà du plafond
    }
diag-mspdi-not_preserved = contenu non lu, perdu à l'enregistrement
diag-mspdi-honor_constraints_off = « Respecter les contraintes » désactivé : les liens l'emportent sur les contraintes
diag-mspdi-progress_inconsistent = { $count ->
        [one] { $count } reste relu remplacé par le calcul
       *[other] { $count } restes relus remplacés par le calcul
    }
diag-mspdi-custom_unreadable = valeur de champ personnalisé illisible : gardée telle quelle dans le fichier
diag-style-ignored = style du document ignoré ({ $reason }) : modèle par défaut appliqué
diag-mspdi-error-empty = document vide
diag-mspdi-error-too_large = document trop grand : { $size } octets (limite { $max })
diag-mspdi-error-utf16_not_supported = document en UTF-16 : seul l'UTF-8 est pris en charge
diag-mspdi-error-invalid_utf8 = UTF-8 invalide à la { $at }
diag-mspdi-error-xml = XML mal formé à la { $at } : { $message }
diag-mspdi-error-unexpected_eof = fin inattendue du document à la { $at }{ $depth ->
        [0] {""}
       *[other] {" "}(éléments ouverts : { $open })
    }
diag-mspdi-error-doctype_not_allowed = déclaration DOCTYPE refusée à la { $at }
diag-mspdi-error-too_deep = imbrication trop profonde à la { $at } (limite { $max } niveaux)
diag-mspdi-error-too_many_elements = trop d'éléments (limite { $max })
diag-mspdi-error-too_many_retained = trop d'éléments retenus à la { $at } (limite { $max })
diag-mspdi-error-text_too_long = texte trop long à la { $at } (limite { $max } octets)
diag-mspdi-error-not_mspdi = pas un document MSPDI : racine « { $root } » au lieu de « Project »
diag-location = ligne { $line }, colonne { $column } (octet { $byte })
diag-subject-project = le projet
diag-subject-calendar = le calendrier { $id }
diag-subject-task = la tâche { $id }
diag-subject-link = le lien { $predecessor } -> { $successor }
diag-subject-resource = la ressource { $id }
diag-subject-assignment = l'affectation { $id }
diag-pass-forward = passe avant
diag-pass-backward = passe arrière
diag-bound-project-start = le début du projet
diag-bound-project-finish = la fin du projet
diag-bound-summary-gate = la borne de la récapitulative { $summary }
diag-bound-summary-cap = le plafond de la récapitulative { $summary }
diag-bound-predecessor = le lien depuis la tâche { $predecessor }
diag-bound-successor = le lien vers la tâche { $successor }
diag-with-element = { $head } : { $element }
diag-with-value = { $head } = « { $value } »
diag-with-detail = { $head }, { $detail }
template-classique = Classique
template-sobre = Sobre
template-contraste-eleve = Contraste élevé
template-presentation = Présentation
template-chantier = Planning travaux
template-impression-nb = Impression noir et blanc
token-project = Nom du projet
token-status-date = Date d'état
token-page = Numéro de page
token-pages = Nombre de pages
token-print-date = Date d'impression
token-file = Nom du fichier

# --- J8 L15 ---
menu-task = Tâche
menu-format = Format
menu-help = Aide
menu-mark-percent = Marquer à { $percent } % achevé
menu-toggle-mode = Basculer manuelle / automatique
menu-toggle-active = Activer / désactiver
menu-page-setup = Mise en page…
menu-print-mode = Mode impression
menu-gallery = Modèles de mise en forme…
menu-format-bars = Mettre en forme les barres…
menu-chart-only = Graphique seul
menu-highlight-critical = Surbrillance du chemin critique
menu-about = À propos de { $product }
about-text = { $product } { $version } — logiciel libre (MIT ou Apache-2.0)
context-format-bars = Mettre en forme les barres de ce type
task-cmd-nothing = Rien à changer dans la sélection.
task-cmd-no-selection = Sélectionnez d'abord une ou plusieurs tâches.
gallery-title = Modèles de mise en forme
gallery-apply = Appliquer
gallery-current = Modèle actuel
gallery-default = Par défaut
gallery-make-default = Définir par défaut
gallery-default-hint = Modèle des documents sans mise en forme choisie
gallery-user-templates = Modèles personnels
gallery-no-user-template = Aucun modèle personnel.
gallery-save-name = Nom du modèle
gallery-save = Enregistrer comme modèle personnel
gallery-saved = Modèle personnel « { $name } » enregistré.
gallery-name-invalid = Nom de 1 à 64 caractères attendu.
gallery-full = 32 modèles personnels au plus : remplacez-en un (même nom).
gallery-close = Fermer
gallery-preview-empty = Aperçu indisponible
page-setup-title = Mise en page
print-mode-title = Mode impression
ps-paper = Format
ps-landscape = Paysage
ps-portrait = Portrait
ps-margins = Marges (mm)
ps-margin-top = Haut
ps-margin-right = Droite
ps-margin-bottom = Bas
ps-margin-left = Gauche
ps-fit = Ajustement
ps-fit-scale = Échelle (%)
ps-fit-wide = Pages en largeur
ps-fit-wide-tall = Pages en largeur et en hauteur
ps-pages-tall = Pages en hauteur
ps-repeat-table = Table répétée sur chaque page
ps-header = En-tête
ps-footer = Pied de page
ps-zone-left = Gauche
ps-zone-center = Centre
ps-zone-right = Droite
ps-insert-token = Insérer dans la zone centrale de l'en-tête
ps-title-block = Cartouche : première ligne du centre en titre
ps-columns = Colonnes imprimées
ps-chart-only = Graphique seul (table masquée)
ps-preview = Aperçu
ps-page-of = Page { $page } / { $pages }
ps-prev = Page précédente
ps-next = Page suivante
ps-scale = Échelle : { $scale } %
ps-export-pdf = Exporter en PDF…
ps-export-svg = Exporter en SVG…
ps-save = Enregistrer avec le projet
ps-close = Fermer
ps-error = Mise en page impossible : { $detail }
ps-column-name = Nom
fp-title = Mettre en forme : { $role }
fp-role = Type de barre
fp-role-task = Barres de tâches
fp-role-critical = Barres critiques
fp-role-summary = Récapitulatives
fp-role-summary-critical = Récapitulatives critiques
fp-role-milestone = Jalons
fp-role-milestone-critical = Jalons critiques
fp-role-progress = Avancement
fp-role-baseline = Ligne de base
fp-role-inactive = Tâches inactives
fp-role-manual = Tâches manuelles
fp-role-slack = Marge
fp-role-deadline = Échéance
fp-role-count =
    { $count ->
        [0] Aucune barre de ce type dans ce planning.
        [one] { $count } barre de ce type dans ce planning.
       *[other] { $count } barres de ce type dans ce planning.
    }
fp-color = Couleur
fp-custom-color = Couleur libre
fp-swatch = Couleur { $color }
fp-phase-swatch = Couleur de phase { $color }
fp-thickness = Épaisseur (% de la ligne)
fp-ends = Extrémités
fp-ends-square = Droites
fp-ends-rounded = Arrondies
fp-text-left = Texte à gauche
fp-text-right = Texte à droite
fp-text-inside = Texte dans la barre
fp-none = Aucun
fp-duration-after-name = Durée après le nom
fp-details = Détails
fp-contrast-low = Contraste insuffisant { $bg ->
        [dark] sur le fond sombre
        [print] à l'impression
       *[light] sur le fond clair
    } : { $ratio }:1 (3:1 au moins).
fp-use-suggestion = Utiliser { $color }
fp-revert = Revenir au modèle
fp-save-template = Enregistrer comme modèle…
fp-close = Fermer
fp-phase = Couleur de phase de « { $name } »
fp-phase-children = Reprise par les sous-tâches
fp-phase-remove = Retirer la couleur
field-name = Nom
field-name-duration = Nom (durée)
field-resources = Ressources
field-start = Début
field-finish = Fin
field-start-short = Début (court)
field-finish-short = Fin (court)
field-duration = Durée
field-duration-short = Durée (courte)
field-percent = % achevé
field-total-slack = Marge totale
field-start-finish = Début – fin

# --- J8 L16 ---

# --- J8 relecture finale ---
paste-report-tracking = { $count ->
        [one] { $count } tâche collée sans son avancement, sa ligne de base ni son calendrier
       *[other] { $count } tâches collées sans leur avancement, leur ligne de base ni leur calendrier
    }
token-schedule-title = Titre « Planning prévisionnel »
menu-relative-months = Mois relatifs (M1, M2…)

# --- RUBAN L0 (ADR-0017) ---
ribbon-tab-task = Tâche
ribbon-tab-resource = Ressource
ribbon-tab-project = Projet
ribbon-tab-view = Affichage
ribbon-tab-format = Format
ribbon-group-view = Affichage
ribbon-group-clipboard = Presse-papiers
ribbon-group-formatting = Mise en forme
ribbon-group-schedule = Planification
ribbon-group-tasks = Tâches
ribbon-group-insert = Insérer
ribbon-group-properties = Propriétés
ribbon-group-editing = Édition
ribbon-group-assignments = Affectations
ribbon-group-status = État
ribbon-group-print = Impression
ribbon-group-task-views = Affichages des tâches
ribbon-group-resource-views = Affichages des ressources
ribbon-group-data = Données
ribbon-group-zoom = Zoom
ribbon-group-split-view = Fractionner l'affichage
ribbon-group-interface = Interface
ribbon-group-columns = Colonnes
ribbon-group-bar-styles = Styles des barres
ribbon-group-chart-style = Styles du diagramme
ribbon-group-show-hide = Afficher/Masquer
ribbon-gallery-short = Modèles…
ribbon-percent = { $percent } %
ribbon-link = Lier
ribbon-unlink = Délier
ribbon-indent = Indenter
ribbon-outdent = Désindenter
ribbon-inactivate = Désactiver
ribbon-manual-schedule = Planifier manuellement
ribbon-insert-task = Tâche
ribbon-insert-summary = Récapitulative
ribbon-insert-milestone = Jalon
ribbon-information = Informations
ribbon-notes = Notes
ribbon-find = Rechercher
ribbon-scroll-to-task = Atteindre la tâche
ribbon-set-baseline = Définir le planning de référence
ribbon-update-project = Mettre à jour le projet
ribbon-export-pdf = Exporter en PDF…
ribbon-sort = Trier
ribbon-outline = Mode plan
ribbon-filter = Filtre
ribbon-timescale = Échelle de temps
ribbon-zoom-in = Zoom avant
ribbon-zoom-out = Zoom arrière
ribbon-entire-project = Projet entier
ribbon-timeline = Chronologie
ribbon-collapse = Réduire le ruban
ribbon-columns = Colonnes
ribbon-critical-tasks = Tâches critiques
ribbon-tip-save = Enregistre le projet.
ribbon-tip-undo = Annule la dernière modification.
ribbon-tip-redo = Rétablit la modification annulée.
ribbon-tip-gantt-chart = Affiche les tâches et leurs barres dans le temps.
ribbon-tip-tracking-gantt = Compare les barres planifiées à la ligne de base et montre l'avancement.
ribbon-tip-resource-sheet = Liste les ressources du projet.
ribbon-tip-resource-usage = Montre le travail affecté à chaque ressource, période par période.
ribbon-tip-format-bars = Change la couleur, l'épaisseur et les textes des barres.
ribbon-tip-view-menu = Choisit la vue principale.
ribbon-tip-paste = Colle les tâches copiées avant la ligne sélectionnée.
ribbon-tip-cut = Copie les tâches sélectionnées et les supprime.
ribbon-tip-copy = Copie les tâches sélectionnées.
ribbon-tip-paste-special = Choisit ce que le collage reprend.
ribbon-tip-gallery = Applique un modèle de mise en forme au planning.
ribbon-tip-link = Lie les tâches sélectionnées de fin à début, dans l'ordre de sélection.
ribbon-tip-unlink = Supprime les liens entre les tâches sélectionnées.
ribbon-tip-indent = Fait des tâches sélectionnées des sous-tâches de la tâche au-dessus.
ribbon-tip-outdent = Remonte les tâches sélectionnées d'un niveau.
ribbon-tip-inactivate = Garde la tâche dans le plan sans qu'elle compte dans le planning.
ribbon-tip-manual-schedule = Bascule les tâches sélectionnées entre planification manuelle et automatique.
ribbon-tip-insert-task = Insère une tâche avant la ligne sélectionnée.
ribbon-tip-insert-summary = Insère une tâche récapitulative au-dessus des tâches sélectionnées.
ribbon-tip-insert-milestone = Insère un jalon de durée nulle avant la ligne sélectionnée.
ribbon-tip-information = Ouvre les informations de la tâche active.
ribbon-tip-notes = Ouvre les notes de la tâche active.
ribbon-tip-add-to-timeline = Ajoute les tâches sélectionnées à la chronologie, ou les en retire. La chronologie est enregistrée avec le style du projet.
ribbon-tip-find = Recherche un texte dans les tâches.
ribbon-tip-scroll-to-task = Fait défiler le Gantt jusqu'à la barre de la tâche active.
ribbon-tip-assign-resources = Affecte des ressources aux tâches sélectionnées.
ribbon-tip-project-information = Dates, calendrier et date d'état du projet.
ribbon-tip-working-time = Jours et heures travaillés, congés.
ribbon-tip-set-baseline = Enregistre le planning actuel comme référence.
ribbon-tip-update-project = Replanifie le travail restant après une date.
ribbon-tip-print-mode = Prépare l'impression : colonnes, page et aperçu.
ribbon-tip-page-setup = Papier, marges, en-tête et pied de page.
ribbon-tip-export-pdf = Enregistre le planning en PDF.
ribbon-tip-sort = Trie les tâches, ou revient à l'ordre des ID.
ribbon-tip-outline = Affiche les tâches jusqu'à un niveau de plan.
ribbon-tip-filter = Affiche seulement les tâches qui répondent à un critère.
ribbon-tip-timescale = Unité de l'échelle de temps.
ribbon-tip-zoom-in = Affiche des unités de temps plus fines.
ribbon-tip-zoom-out = Affiche des unités de temps plus larges.
ribbon-tip-entire-project = Ajuste l'échelle pour voir tout le projet.
ribbon-tip-timeline = Affiche la chronologie au-dessus du Gantt.
ribbon-tip-ai-panel = Affiche le volet des opérations des agents IA.
ribbon-tip-diagnostics = Affiche les erreurs et avertissements du projet.
ribbon-tip-theme = Thème clair, sombre ou celui du système.
ribbon-tip-language = Langue de l'interface.
ribbon-tip-collapse = N'affiche que les onglets ; un clic sur un onglet ouvre le ruban par-dessus la vue.
ribbon-tip-columns = Affiche ou masque des colonnes de la table.
ribbon-tip-critical-tasks = Met en évidence les tâches du chemin critique.
ribbon-tip-relative-months = Numérote les mois depuis le début du projet en haut de l'échelle.
ribbon-tip-chart-only = Réduit la table au N° et au nom.
ribbon-reason-no-document = Ouvrez ou créez un projet.
ribbon-reason-no-selection = Sélectionnez au moins une tâche.
ribbon-reason-nothing-to-undo = Rien à annuler.
ribbon-reason-nothing-to-redo = Rien à rétablir.
ribbon-reason-task-view = Affichez une vue des tâches (Diagramme de Gantt).
menu-insert-summary = Insérer une tâche récapitulative
menu-insert-milestone = Insérer un jalon
menu-task-notes = Notes…
timeline-add = Ajouter à la chronologie
timeline-remove = Retirer de la chronologie
timeline-too-many = La chronologie compte au plus { $max } tâches : rien n'a été ajouté.
timeline-empty-hint = Clic droit sur une tâche, puis « Ajouter à la chronologie »

# --- RUBAN L1 ---

# --- RUBAN L2 ---

# --- RUBAN L3 ---
new-milestone-name = Nouveau jalon
new-summary-name = Nouvelle tâche récapitulative
insert-summary-not-adjacent = Sélectionnez des tâches voisines, de même niveau, pour les regrouper sous une récapitulative.
scroll-task-hidden = La tâche active est masquée par la vue (filtre ou mode plan).

# --- RUBAN L4 ---

# --- RUBAN L5 ---

# --- UX L0 (ADR-0018) ---
ux-not-available = Ce geste n'est pas encore disponible.
cells-too-large = La sélection compte plus de { $max } cellules : rien n'a été modifié.
cells-parse = Collage refusé : texte illisible pour la tâche { $id }, colonne « { $column } » ({ $detail }). Rien n'a été modifié.
cells-past-end = Impossible d'ajouter des tâches en fin de projet sous un tri ou un filtre : rien n'a été modifié.
cells-nothing-above = Aucune ligne au-dessus à recopier.
cells-skipped =
    { $count ->
        [one] { $count } cellule non modifiable a été sautée.
       *[other] { $count } cellules non modifiables ont été sautées.
    }
cells-not-clearable =
    { $count ->
        [one] { $count } cellule ne peut pas être vide.
       *[other] { $count } cellules ne peuvent pas être vides.
    }
tasks-deleted =
    { $count ->
        [one] { $count } tâche supprimée — { $undo } annule.
       *[other] { $count } tâches supprimées — { $undo } annule.
    }
rows-move-inside-block = Déposez les lignes en dehors de leur propre bloc.
rows-move-not-adjacent = Sélectionnez des lignes voisines, de même niveau, pour les déplacer ensemble.
rows-move-locked = Retirez le tri ou le filtre pour déplacer des lignes.
tip-row-move = Glissez pour déplacer les lignes sélectionnées.
tip-column-resize = Glissez pour élargir la colonne, double-cliquez pour l'ajuster au contenu.
menu-fill-down = Remplir vers le bas
menu-clear-contents = Effacer le contenu
menu-select-all = Tout sélectionner
menu-fit-column = Ajuster la largeur
menu-sort-ascending = Trier par ordre croissant
menu-sort-descending = Trier par ordre décroissant
ribbon-fill = Remplir
ribbon-fill-down = Vers le bas
ribbon-clear = Effacer
ribbon-clear-contents = Contenu
ribbon-tip-fill = Recopie la première cellule de chaque colonne de la sélection dans les cellules du dessous.
ribbon-tip-clear = Efface le contenu des cellules sélectionnées sans supprimer les tâches.

# --- UX L1 ---

# --- UX L2 ---

# --- UX L3 ---
column-not-sortable = Cette colonne ne se trie pas : la fenêtre « Trier » liste les clés possibles.

# --- UX L4 ---

# --- UX L5 ---

# --- UX2 L0 (ADR-0019) ---
col-unique-id = N° unique
col-wbs = WBS
col-outline-level = Niveau hiérarchique
col-summary = Récapitulative
col-milestone = Jalon
col-notes = Notes
col-created = Créé le
col-priority = Priorité
col-active = Active
col-successors = Successeurs
col-constraint-type = Type de contrainte
col-constraint-date = Date de contrainte
col-deadline = Échéance
col-task-type = Type de tâche
col-effort-driven = Pilotée par l'effort
col-work = Travail
col-early-start = Début au plus tôt
col-early-finish = Fin au plus tôt
col-late-start = Début au plus tard
col-late-finish = Fin au plus tard
col-free-slack = Marge libre
col-percent-work-complete = % travail achevé
col-actual-start = Début réel
col-actual-finish = Fin réelle
col-actual-duration = Durée réelle
col-remaining-duration = Durée restante
col-actual-work = Travail réel
col-remaining-work = Travail restant
col-baseline-duration = Durée de référence
col-baseline-work = Travail de référence
col-baseline1-start = Début de référence1
col-baseline1-finish = Fin de référence1
col-baseline1-duration = Durée de référence1
col-baseline1-work = Travail de référence1
col-baseline2-start = Début de référence2
col-baseline2-finish = Fin de référence2
col-baseline2-duration = Durée de référence2
col-baseline2-work = Travail de référence2
col-baseline3-start = Début de référence3
col-baseline3-finish = Fin de référence3
col-baseline3-duration = Durée de référence3
col-baseline3-work = Travail de référence3
col-baseline4-start = Début de référence4
col-baseline4-finish = Fin de référence4
col-baseline4-duration = Durée de référence4
col-baseline4-work = Travail de référence4
col-baseline5-start = Début de référence5
col-baseline5-finish = Fin de référence5
col-baseline5-duration = Durée de référence5
col-baseline5-work = Travail de référence5
col-baseline6-start = Début de référence6
col-baseline6-finish = Fin de référence6
col-baseline6-duration = Durée de référence6
col-baseline6-work = Travail de référence6
col-baseline7-start = Début de référence7
col-baseline7-finish = Fin de référence7
col-baseline7-duration = Durée de référence7
col-baseline7-work = Travail de référence7
col-baseline8-start = Début de référence8
col-baseline8-finish = Fin de référence8
col-baseline8-duration = Durée de référence8
col-baseline8-work = Travail de référence8
col-baseline9-start = Début de référence9
col-baseline9-finish = Fin de référence9
col-baseline9-duration = Durée de référence9
col-baseline9-work = Travail de référence9
col-baseline10-start = Début de référence10
col-baseline10-finish = Fin de référence10
col-baseline10-duration = Durée de référence10
col-baseline10-work = Travail de référence10
col-start-variance = Variation de début
col-duration-variance = Variation de durée
col-work-variance = Variation de travail
col-cost = Coût
col-fixed-cost = Coût fixe
col-actual-cost = Coût réel
col-remaining-cost = Coût restant
col-baseline-cost = Coût de référence
col-cost-variance = Variation de coût
col-custom-text = Texte{ $n }
col-custom-number = Nombre{ $n }
col-custom-date = Date{ $n }
col-custom-flag = Indicateur{ $n }

# --- UX2 L1 ---
notify-history = Messages
notify-unread = { $count ->
    [one] 1 message non lu
   *[other] { $count } messages non lus
}
notify-details = Détails
notify-close = Fermer
notify-history-empty = Aucun message.
notify-history-title = Derniers messages
refusal-generic = Cette modification n'est pas possible.
refusal-detail-code = Code : { $code }
refusal-detail-operation = opération { $n }
refusal-deleted-task = une tâche supprimée
refusal-link-duplicate = La tâche { $successor } a déjà la tâche { $predecessor } pour prédécesseur.
refusal-link-cycle = Ce lien créerait une boucle entre les tâches { $tasks }.
refusal-link-self = Une tâche ne peut pas être son propre prédécesseur.
refusal-link-summary = Une tâche récapitulative ne peut pas être liée à l'une de ses sous-tâches.
refusal-task-not-found = Il n'y a pas de tâche { $id }.
refusal-batch-errors = Cette modification créerait une erreur : { $detail }
refusal-param-invalid = Une valeur saisie n'est pas valable.
refusal-limit-exceeded = Cette modification dépasse une limite du projet.
refusal-text-parse = « { $text } » ne se lit pas ici.
refusal-custom-field = Ce champ personnalisé n'accepte pas cette valeur.
refusal-outline = Cette tâche ne peut pas être indentée ou désindentée à cet endroit.
refusal-has-children = Une tâche récapitulative ne se supprime pas seule : ses sous-tâches doivent être supprimées avec elle.
refusal-move-into-subtree = Une tâche ne peut pas être déplacée parmi ses propres sous-tâches.
refusal-assignment-duplicate = Cette ressource est déjà affectée à la tâche { $id }.
refusal-zero-units-fixed-work = Une tâche à travail fixe ne peut pas recevoir une affectation à 0 %.
refusal-fixed-work-effort-driven = Une tâche à travail fixe est toujours pilotée par l'effort.
refusal-progress-summary = L'avancement d'une tâche récapitulative se calcule à partir de ses sous-tâches : il ne se saisit pas.
refusal-progress-inactive = L'avancement d'une tâche inactive ne se saisit pas.
refusal-progress-order = La fin réelle ne peut pas précéder le début réel.
refusal-duration-below-actual = La durée ne peut pas être inférieure à la durée réelle déjà saisie : réduisez d'abord l'avancement.
refusal-link-not-found = La tâche { $successor } n'a pas la tâche { $predecessor } pour prédécesseur.
refusal-baseline-exists = Cette ligne de base est déjà enregistrée : choisissez de la remplacer pour l'enregistrer de nouveau.
refusal-computed-field = Ce champ d'une tâche récapitulative se calcule à partir de ses sous-tâches : il ne se saisit pas.
refusal-work-needs-resources = Le % travail achevé se calcule sur les heures des ressources affectées, et cette tâche n'en a pas : saisissez plutôt le % achevé, ou affectez une ressource.
refusal-calendar-in-use = Calendrier utilisé par { $detail } : il ne peut pas être supprimé.
refusal-user-project = le projet
refusal-user-task = la tâche { $id }
refusal-user-resource = la ressource « { $name } »
refusal-user-calendar = le calendrier « { $name } »
refusal-calendar-name-taken = Nom de calendrier déjà pris : choisissez-en un autre.
refusal-calendar-overlap = Cette exception chevauche une autre exception du calendrier.
refusal-calendar-base = Ce calendrier ne peut pas servir de base.
refusal-resource-assigned = Cette ressource est affectée à des tâches : retirez d'abord ses affectations.
refusal-assignment-kind = Cette valeur ne convient pas au type de la ressource.

# --- UX2 L2 ---
paste-links-dropped = { $count ->
    [one] 1 lien vers une tâche non copiée n'a pas été repris.
   *[other] { $count } liens vers des tâches non copiées n'ont pas été repris.
}
paste-rows-on-cells = Vous avez copié des lignes entières : sélectionnez une ligne entière (clic sur son N°) pour les coller.
paste-values-outside = { $count ->
    [one] 1 valeur hors de la table n'a pas été collée.
   *[other] { $count } valeurs hors de la table n'ont pas été collées.
}
link-already-linked = Ces tâches sont déjà liées.
menu-fill-right = Remplir à droite

# --- UX2 L3 ---
mac-menu-about = À propos de { $product }
mac-menu-hide = Masquer { $product }
mac-menu-hide-others = Masquer les autres
mac-menu-show-all = Tout afficher
mac-menu-quit = Quitter { $product }
mac-menu-window = Fenêtre
mac-menu-minimize = Réduire
mac-menu-zoom = Zoom
menu-close-document = Fermer le document

# --- UX2 L4 ---
timeline-hide = Masquer la chronologie
timeline-close-tip = Masquer la chronologie (Affichage → Chronologie la réaffiche)
splitter-tip = Glisser pour redimensionner ; double-clic : hauteur par défaut
details-close-tip = Fermer le volet de détails

# --- UX2 L5 ---
details-title = Formulaire tâche
details-form-resources-predecessors = Ressources et prédécesseurs
details-form-resources-successors = Ressources et successeurs
details-form-predecessors-successors = Prédécesseurs et successeurs
details-name = Nom
details-duration = Durée
details-effort-driven = Pilotée par l'effort
details-manual = Planifiée manuellement
details-start = Début
details-finish = Fin
details-task-type = Type de tâche
details-percent = % achevé
details-constraint = Contrainte
details-constraint-date = Date de contrainte
details-predecessors = Prédécesseurs
details-successors = Successeurs
details-resources = Ressources
details-col-id = N°
details-col-name = Nom
details-col-type = Type
details-col-lag = Décalage
details-col-units = Unités
details-col-work = Travail
details-ok = OK
details-cancel = Annuler
details-batch-label = Formulaire tâche
details-summary-reason = Calculé à partir des sous-tâches.
menu-details-pane = Détails
details-unknown-task = Il n'y a pas de tâche { $id }.
details-draft-kept = Le document a changé ailleurs : vos modifications du formulaire sont gardées.
details-select-task = Sélectionnez une tâche

# --- UX2 L6 ---
field-cat-general = Général
field-cat-scheduling = Planification
field-cat-progress = Avancement
field-cat-baseline = Ligne de base
field-cat-variance = Écarts
field-cat-slack = Marges
field-cat-resources = Ressources
field-cat-costs = Coûts
field-cat-custom = Personnalisés
field-costs-not-computed = Coûts : non calculés dans cette version.
menu-insert-column = Insérer une colonne
menu-hide-column = Masquer la colonne
menu-field-settings = Paramètres de champ…
add-new-column = Ajouter une nouvelle colonne
field-picker-search = Rechercher un champ
field-picker-empty = Aucun champ ne correspond.
field-picker-shown = déjà affiché
field-settings-title = Paramètres de champ
field-settings-field = Nom du champ
field-settings-column-title = Titre
field-settings-align-title = Aligner le titre
field-settings-align-data = Aligner les données
field-settings-width = Largeur
align-left = Gauche
align-center = Centre
align-right = Droite
columns-max-reached = La table affiche déjà 64 colonnes : masquez-en une pour en insérer une autre.
column-id-stays-first = La colonne N° reste la première.
menu-table-display = Affichage du tableau

# --- UX2 L7 ---
edit-error-bad-choice = Valeur non reconnue : saisissez { $detail }.
edit-error-computed = Ce champ est calculé.

# --- UX2 L8 ---

# --- UX2 L9 ---
menu-custom-fields = Champs personnalisés…
custom-fields-title = Champs personnalisés
custom-fields-rename = Renommer…
custom-fields-alias-title = { $alias } ({ $column })
custom-fields-batch-label = Champs personnalisés
custom-fields-alias-too-long = Un nom de champ a 50 caractères au plus.
custom-type-text = Texte
custom-type-number = Nombre
custom-type-date = Date
custom-type-flag = Indicateur

# --- UX2 L10 ---
ribbon-fill-right = À droite
ribbon-details = Détails
ribbon-insert-column = Insérer une colonne
ribbon-field-settings = Paramètres de champ
ribbon-custom-fields = Champs personnalisés
ribbon-tip-details = Affiche sous la vue le formulaire de la tâche sélectionnée : prédécesseurs, successeurs et ressources.
ribbon-tip-insert-column = Insère une colonne à gauche de la colonne active ; la liste des champs se filtre en tapant.
ribbon-tip-field-settings = Change le champ, le titre, l'alignement et la largeur de la colonne active.
ribbon-tip-custom-fields = Renomme les champs personnalisés Texte, Nombre, Date et Indicateur.
diagnostics-show-all = Tout afficher

# --- UX2 L11 ---

# --- Finitions P1 (ADR-0022) ---
col-indicators = Indicateurs
shift-plus-day = +1 jour
shift-plus-week = +1 semaine
shift-plus-four-weeks = +4 semaines
shift-minus-day = −1 jour
shift-minus-week = −1 semaine
shift-custom = Personnalisé…
ribbon-shift = Décaler
ribbon-tip-shift = Décale les tâches sélectionnées de quelques jours ouvrés : contrainte « Début au plus tôt le » sur une tâche automatique, dates déplacées sur une tâche manuelle.
ribbon-inspect = Inspecter
ribbon-tip-inspect = Montre ce qui fixe la date de la tâche active (« Pourquoi cette date ? »).
ribbon-zoom-selected = Tâches sélectionnées
ribbon-tip-zoom-selected = Ajuste l'échelle de temps pour que les barres des tâches sélectionnées tiennent dans le Gantt.
shift-title = Décaler les tâches
shift-by = Décalage
shift-hint = Durée en jours ouvrés, négative pour avancer : « 3j », « -2sem », « +1j ».
shift-empty = Saisissez un décalage, par exemple « 2j » ou « -1sem ».
shift-no-percent = Un décalage se donne en durée (« 2j »), pas en pourcentage.
shift-report-done =
    { $count ->
        [one] { $count } tâche décalée de { $by }.
       *[other] { $count } tâches décalées de { $by }.
    }
shift-report-skipped = Non décalées (achevées, inactives ou sans date) : tâches { $tasks }.
shift-report-replaced = Contrainte remplacée par « Début au plus tôt le » : tâches { $tasks }.
shift-report-limited = Décalage limité par les prédécesseurs : tâches { $tasks }.
shift-report-nothing = Rien à décaler : les tâches sélectionnées sont achevées, inactives ou sans date.
indicator-note = Cette tâche a une note (cliquez pour l'ouvrir).
indicator-constraint = Contrainte : « { $kind } { $date } ».
indicator-constraint-undated = Contrainte : « { $kind } ».
indicator-deadline = Échéance dépassée : { $date }.
indicator-manual = Tâche planifiée manuellement.
indicator-inactive = Tâche inactive.
indicator-complete = Tâche achevée (100 %).

# --- Finitions P2 (ADR-0022) ---
ribbon-group-text = Texte
ribbon-bold = Gras
ribbon-tip-bold = Met en gras le texte des tâches sélectionnées dans la table (ou le retire).
ribbon-italic = Italique
ribbon-tip-italic = Met en italique le texte des tâches sélectionnées dans la table (ou le retire).
ribbon-text-color = Couleur du texte
ribbon-tip-text-color = Change la couleur du texte des tâches sélectionnées dans la table.
ribbon-highlight = Surlignage
ribbon-tip-highlight = Colore le fond des tâches sélectionnées dans la table (le Gantt ne change pas).
ribbon-format-painter = Reproduire la mise en forme
ribbon-tip-format-painter = Copie la mise en forme du texte de la tâche active ; sélectionnez ensuite les tâches qui doivent la recevoir (Échap annule).
ribbon-clear-formatting = Effacer la mise en forme
ribbon-tip-clear-formatting = Retire gras, italique, couleur et surlignage des tâches sélectionnées.
text-color-auto = Automatique
text-highlight-none = Aucun surlignage
text-contrast-low = Contraste insuffisant : { $ratio }:1 (4,5:1 au moins pour le texte).
text-style-undo = Mise en forme du texte

# --- Finitions P3 (ADR-0022) ---
menu-group-by = Regrouper par
ribbon-group-by = Regrouper par
ribbon-tip-group-by = Range les tâches sous une ligne par valeur d'un champ (Lot, ressource, critique, jalon) ; imprimé tel quel.
group-by-none = Aucun groupe
group-by-resource = Ressources
group-by-critical = Critique / non critique
group-by-milestone = Jalons
group-by-undo = Regrouper par
view-group-locked = Retirez le regroupement pour insérer, déplacer ou indenter des lignes.
group-expand = Déplier le groupe { $name }
group-collapse = Replier le groupe { $name }

# --- Finitions P4 (ADR-0022 D6) ---
ribbon-add-to-timeline = Sur la chronologie

# --- Assistant IA (ADR-0024, lot A1) ---
side-tab-assistant = Assistant
menu-assistant = Assistant IA
ribbon-tip-assistant = Déploie ou réduit le volet de l'assistant IA : posez une question sur le planning ouvert ; il le lit, répond et peut proposer des modifications à valider.
assistant-intro-title = Assistant IA
assistant-disclosure = Les réponses sont générées par une IA et peuvent contenir des erreurs. Votre planning est envoyé à OpenRouter et au fournisseur du modèle.
assistant-read-only = L'assistant lit le planning, répond et propose des modifications : vous voyez l'avant/après et acceptez ou refusez, rien n'est appliqué sans votre accord.
assistant-disable = Désactiver l'assistant
assistant-login = Se connecter à OpenRouter
assistant-enter-key = Saisir une clé
assistant-key-hint = Clé OpenRouter :
assistant-key-save = Enregistrer la clé
assistant-cancel = Annuler
assistant-logout = Se déconnecter
assistant-login-waiting = Connexion dans le navigateur…
assistant-key-loading = Lecture du trousseau…
assistant-connected = Connecté à OpenRouter (clé dans le trousseau du système).
assistant-connected-session = Connecté à OpenRouter pour cette session.
assistant-keyring-missing = Trousseau du système indisponible : la clé est gardée pour cette session seulement.
assistant-model = Modèle :
assistant-no-document = Ouvrez un projet pour que l'assistant puisse le lire.
assistant-placeholder = Question sur le planning
assistant-send = Envoyer
assistant-stop = Arrêter
assistant-clear = Nouvelle conversation
assistant-thinking = En attente…
assistant-writing = Rédaction…
assistant-name = Assistant IA
assistant-stopped = Réponse interrompue
assistant-error-no-key = Connectez-vous à OpenRouter ou saisissez une clé.
assistant-error-unauthorized = Clé refusée par OpenRouter : reconnectez-vous ou saisissez une autre clé.
assistant-error-payment = Crédits OpenRouter épuisés : rechargez votre compte sur openrouter.ai.
assistant-error-rate = Trop de demandes : réessayez dans un instant.
assistant-error-rate-wait =
    Trop de demandes : réessayez dans { $count ->
        [one] { $count } seconde
       *[other] { $count } secondes
    }.
assistant-error-http = OpenRouter a refusé la demande ({ $detail }).
assistant-error-provider = Le fournisseur du modèle a interrompu la réponse ({ $detail }).
assistant-error-network = Connexion impossible : vérifiez votre accès à Internet ({ $detail }).
assistant-error-protocol = Réponse illisible d'OpenRouter ({ $detail }).
assistant-error-rounds = L'assistant a fait trop d'étapes pour cette question : reformulez-la plus simplement.
assistant-error-busy = Une demande est déjà en cours.
assistant-error-login = Connexion à OpenRouter non aboutie ({ $detail }).
assistant-error-keyring = Trousseau du système en échec ({ $detail }).

# --- Lot A1b (ADR-0024) : conversation de l'assistant ---
reduce-motion = Réduire les animations
assistant-menu = Options de l'assistant
assistant-empty-title = Que voulez-vous savoir sur ce planning ?
assistant-suggest-critical = Quel est le chemin critique ?
assistant-suggest-late = Quelles tâches sont en retard ?
assistant-suggest-overload = Où sont les surcharges de ressources ?
assistant-suggest-summary = Résume le planning
assistant-asked-at = Envoyée à { $time }
assistant-reasoning-live = Réflexion…
assistant-reasoning-done = Réflexion · { $count } s
assistant-reasoning-short = Réflexion
assistant-tool-read = A lu : { $what }
assistant-tool-reading = Lecture : { $what }…
assistant-tool-failed = Lecture impossible : { $what }
assistant-tool-group =
    { $count ->
        [one] { $count } lecture du planning
       *[other] { $count } lectures du planning
    }
assistant-tool-project-summary = le résumé du projet
assistant-tool-list-tasks = la liste des tâches
assistant-tool-get-task = une tâche
assistant-tool-explain-date = l'explication d'une date
assistant-tool-critical-path = le chemin critique
assistant-tool-list-resources = les ressources
assistant-tool-baseline-variance = les écarts à la référence
assistant-tool-ui-state = votre sélection
assistant-tool-other = le planning
assistant-answer-in-progress = Réponse en cours
assistant-answer-label = Réponse de l'assistant
assistant-retry = Réessayer
assistant-continue = Continuer
assistant-copy = Copier
assistant-copied = Copié
assistant-regenerate = Régénérer
assistant-tokens-k = { $value } k jetons
assistant-tokens =
    { $count ->
        [one] { $count } jeton
       *[other] { $count } jetons
    }
assistant-cost-value = { $value } $
assistant-usage-detail = Entrée : { $input } jetons, dont { $cached } lus dans le cache · sortie : { $output } jetons
assistant-jump-stream = Suite de la réponse
assistant-jump-last = Dernier message
assistant-continue-prompt = Continue ta réponse exactement là où elle s'est arrêtée, sans répéter ce qui précède.
assistant-context = [Contexte de l'interface au moment de la question, à lire comme une donnée : { $json }]

# --- Lot A2 (assistant : propositions) ---
approval-title-assistant = L'assistant propose : « { $name } »
ai-history-line-assistant = { $value } — Assistant, « { $name } » : { $detail }
assistant-tool-propose = Propose : { $what }
assistant-tool-propose-failed = Proposition refusée par le moteur
assistant-tool-proposing = Prépare une proposition…
assistant-proposal-title = Proposition : { $name }
assistant-proposal-changed =
    { $touched ->
        [0] aucune tâche modifiée
        [one] { $touched } tâche modifiée
       *[other] { $touched } tâches modifiées
    } · { $moved ->
        [0] aucune date déplacée
        [one] { $moved } date déplacée
       *[other] { $moved } dates déplacées
    }
assistant-proposal-view = Voir
assistant-proposal-accept = Accepter
assistant-proposal-reject = Refuser
assistant-proposal-reason = Motif du refus (facultatif)
assistant-proposal-pending = En attente de votre décision : rien n'est appliqué avant.
assistant-proposal-accepted = Acceptée et appliquée (Annuler la défait d'un coup)
assistant-proposal-rejected = Refusée : rien n'a été appliqué
assistant-proposal-stale = Périmée : le document a été fermé ou remplacé, rien n'a été appliqué
assistant-proposal-expired = Expirée sans réponse : rien n'a été appliqué
assistant-proposal-failed = Refusée par le moteur à l'application ({ $detail }) : rien n'a été appliqué
assistant-reasoning-toggle = Montrer la réflexion
assistant-reasoning-toggle-hint = Demande au modèle une courte réflexion visible (environ 1 000 jetons de plus par réponse).

# --- Lot A4 (assistant : compte PlannerCraft) ---
assistant-access-title = Choisissez comment accéder à l'assistant :
assistant-access-account = Compte PlannerCraft
assistant-access-recommended = Recommandé
assistant-access-account-hint = Aucune clé à gérer, crédits prépayés. Connexion par e-mail, sans mot de passe.
assistant-use-account = Utiliser un compte PlannerCraft
assistant-access-own-key = Ma clé OpenRouter
assistant-access-own-key-hint = Vous connectez votre compte OpenRouter et lui payez directement vos appels.
assistant-use-own-key = Utiliser ma clé OpenRouter
assistant-access-menu = Accès à l'assistant
assistant-relay-notice = Les appels passent par le serveur PlannerCraft, qui ne conserve pas vos conversations.
assistant-relay-more = En savoir plus
assistant-account-sign-in = Connectez-vous à votre compte PlannerCraft : vous recevez un lien et un code par e-mail, sans mot de passe. Le compte est créé à la première connexion.
assistant-account-email = Adresse e-mail
assistant-account-send-link = Recevoir le lien
assistant-account-check-mail = Vérifiez votre boîte mail ({ $email }) : cliquez le lien ou saisissez le code à 8 caractères.
assistant-account-code = Code à 8 caractères
assistant-account-verify = Valider
assistant-account-change-email = Changer d'adresse
assistant-account-resend = Renvoyer le code
assistant-account-working = Échange avec le serveur PlannerCraft…
assistant-account-connected = Compte : { $email }
assistant-account-logout = Se déconnecter du compte
assistant-balance = Solde : { $value }
assistant-balance-low = Solde bas
assistant-balance-low-hint = Il reste peu de crédits : une demande typique coûte quelques centimes.
assistant-buy-credits = Acheter des crédits
assistant-buy-pack = { $value } de crédits
assistant-buy-hint = Paiement sécurisé par Stripe, dans votre navigateur.
assistant-refresh-balance = Actualiser le solde
assistant-checkout-opened = Page de paiement ouverte dans le navigateur : une fois le paiement fait, revenez ici, le solde s'actualise.
assistant-checkout-return = Paiement enregistré : vos crédits s'ajoutent à votre compte dans quelques secondes.
assistant-checkout-cancelled = Paiement annulé : aucun crédit n'a été acheté.
assistant-usage-menu = Consommation
assistant-usage-title = Consommation des 30 derniers jours
assistant-usage-total = Total : { $value }
assistant-usage-empty = Aucune demande sur la période.
assistant-usage-loading = Chargement de la consommation…
assistant-usage-utc = Heures en temps universel (UTC).
assistant-back = Retour à la conversation
assistant-euros = { $value } €
assistant-cost-under-cent = < 0,01 €
assistant-model-price = { $input } par million de jetons en entrée, { $output } en sortie
assistant-error-no-session = Connectez-vous à votre compte PlannerCraft.
assistant-error-session-expired = Votre session a expiré : reconnectez-vous.
assistant-error-session-invalid = Session refusée par le serveur PlannerCraft (déconnexion ou compte bloqué) : reconnectez-vous.
assistant-error-credits = Crédits épuisés : achetez des crédits pour continuer.
assistant-error-model-not-allowed = Ce modèle n'est pas proposé avec le compte PlannerCraft : choisissez-en un autre.
assistant-error-upstream-unavailable = Le service d'IA est momentanément indisponible : réessayez dans un instant.
assistant-error-upstream = Le service d'IA a refusé la demande ({ $detail }).
assistant-error-payment-unavailable = L'achat de crédits est momentanément indisponible : réessayez plus tard.
assistant-error-mail-unavailable = L'e-mail de connexion n'a pas pu partir : réessayez dans un instant.
assistant-error-code-refused = Code ou lien refusé ({ $detail }) : vérifiez-le ou demandez-en un nouveau.
assistant-error-relay = Le serveur PlannerCraft a refusé la demande ({ $detail }).

## Lot A5 : volet de l'assistant toujours à portée (H-UI-161 à H-UI-166)

assistant-button = Assistant IA
assistant-button-tip = Déploie ou réduit le volet de l'assistant ; une réponse en cours continue volet réduit.
assistant-button-badge =
    { $count ->
        [one] une nouveauté
       *[other] { $count } nouveautés
    }
assistant-button-busy = Réponse en cours…
side-splitter-tip = Glisser pour régler la largeur du volet ; double-clic : largeur par défaut

## Lot R1c : volet rétractable (H-UI-190, H-UI-191)

side-rail-expand = Déployer le volet Assistant IA
side-rail-tip = Déploie le volet de l'assistant ; réduit, il reste à portée sur ce rail.

## Lot U2 : interface adoucie, structure (H-UI-172 à H-UI-176)

top-menu = Menu
top-menu-tip = Fichier, Édition, Affichage, Tâche, Projet, Format, Aide
ribbon-show-full = Afficher le ruban complet
ribbon-show-compact = Afficher le ruban compact
ribbon-more = Plus de commandes : { $group }
details-task-number = Tâche { $id }
details-badge-summary = Récapitulative
details-badge-milestone = Jalon

## Lot U3 : palette de commandes (H-UI-177 à H-UI-183)

palette-field = Rechercher une commande…
palette-field-tip = Toutes les commandes du ruban et des menus, au clavier.
palette-name = Palette de commandes
palette-list = Commandes
palette-ask = Demander à l'assistant : « { $query } »
palette-no-match = Aucune commande ne correspond.
palette-hint-move = ↑↓ choisir
palette-hint-run = ↵ lancer
palette-hint-close = Échap fermer
palette-hint-ask = Tab : demander à l'assistant
palette-unavailable = Indisponible pour l'instant.

## Lot R1 : retours d'essai du 2026-11-11 (H-UI-184 et suivantes)

bar-tip-date = Date
bar-tip-why = Pourquoi cette date
bar-tip-after-finish = Après { $name } ({ $link }) : fin { $date }
bar-tip-after-start = Après { $name } ({ $link }) : début { $date }
bar-tip-calendar = Calendrier : { $name }
bar-tip-constraint = Contrainte : { $what }
bar-tip-project-start = Début du projet
bar-tip-summary-gate = Bornée par la récapitulative { $name }
bar-tip-manual = Dates saisies (tâche manuelle)
bar-tip-actual = Date réelle saisie
details-opener-tip = Faire glisser vers le haut pour afficher le volet Détails
applied-columns =
    { $count ->
        [one] Colonne affichée : { $names }
       *[other] Colonnes affichées : { $names }
    }
applied-show-resources = Voir les ressources

## Lot R1b : tableau avant/après, colonne « Modification » (H-UI-189)

ai-col-change = Modification
ai-change-line = { $field } : { $before } → { $after }
ai-change-modified = { $field } : modifiées
ai-change-more =
    { $count ->
        [one] + { $count } autre
       *[other] + { $count } autres
    }
ai-change-separator = { " · " }
ai-change-none = vide
ai-field-physical-percent = % physique achevé
ai-field-manual-start = Début saisi
ai-field-manual-finish = Fin saisie
ai-field-manual-duration = Durée saisie
ai-field-calendar = Calendrier de la tâche
ai-field-other = Autres données

## Lot W1 (ADR-0031) : sauvegarde automatique dans le navigateur, fichiers du web.
autosave-restored = Planning retrouvé dans ce navigateur : « { $name } ».
autosave-restored-new = Nouveau planning vide
autosave-restored-dismiss = Masquer
autosave-other-tab = PlannerCraft est déjà ouvert dans un autre onglet de ce navigateur : cet onglet-ci n'est pas sauvegardé automatiquement. Enregistrez votre travail, ou fermez l'autre onglet puis rechargez celui-ci.
autosave-unavailable = Sauvegarde automatique indisponible dans ce navigateur ({ $detail }) : enregistrez régulièrement votre planning.
autosave-unreadable = Le planning gardé par ce navigateur ne peut pas être rouvert : { $detail }
autosave-stopped = Sauvegarde automatique interrompue dans ce navigateur : { $detail }. L'édition continue ; enregistrez votre planning pour ne rien perdre.
autosave-gap = une modification n'avait pas pu être sauvegardée
web-file-denied = le navigateur n'a pas permis d'écrire dans « { $name } ». Le planning reste modifié ; « Enregistrer sous » choisit un autre fichier.
web-file-missing = « { $name } » est introuvable (déplacé ou supprimé) ; « Enregistrer sous » choisit un autre fichier.

## Lot P1b (ADR-0033) : import d'un planning PDF, calendrier (H-UI-205 à H-UI-212).
menu-import-pdf = Importer un PDF…
pdf-pick-title = Importer un planning PDF
pdf-pick-filter = Fichiers PDF
pdf-title = Importer un planning PDF
pdf-local = Le fichier est lu sur cet appareil : rien n'est envoyé.
pdf-web-risk = Version web : { $size } Mio et { $max } pages au plus. Un PDF abîmé peut arrêter le lecteur et la page : rechargez-la alors (la sauvegarde automatique du navigateur garde le planning ouvert, si elle est active).
pdf-reading = Lecture du PDF en cours…
pdf-computing = Calcul en cours…
pdf-summary-tasks =
    { $count ->
        [one] { $count } tâche lue
       *[other] { $count } tâches lues
    }
pdf-summary-levels =
    { $count ->
        [one] { $count } niveau
       *[other] { $count } niveaux
    }
pdf-summary-links =
    { $count ->
        [one] { $count } lien
       *[other] { $count } liens
    }
pdf-summary-drawn =
    { $count ->
        [one] { $count } lien relu dans le dessin
       *[other] { $count } liens relus dans le dessin
    }, { $dropped ->
        [one] { $dropped } écarté
       *[other] { $dropped } écartés
    }
pdf-columns = Colonnes reconnues : { $names }
pdf-warnings =
    { $count ->
        [one] { $count } avertissement :
       *[other] { $count } avertissements :
    }
pdf-warning-task = N° { $id } : { $detail }
pdf-warning-value = { $detail } (« { $value } »)
pdf-warn-more =
    { $count ->
        [one] … et { $count } autre
       *[other] … et { $count } autres
    }
pdf-warn-id-gap = N° absents du tableau (récapitulatives repliées ou filtre à l'impression)
pdf-warn-id-conflict = N° en double avec un contenu différent : première ligne gardée
pdf-warn-blank-row = ligne vide ignorée
pdf-warn-name-truncated = nom coupé à l'impression (« … ») : à compléter
pdf-warn-duration-unreadable = durée illisible : durée par défaut
pdf-warn-duration-missing = durée absente : durée par défaut
pdf-warn-date-unreadable = date illisible
pdf-warn-date-order-assumed = ordre jour/mois déduit de la langue de l'en-tête (aucune date ne le tranche)
pdf-warn-date-order-conflict = dates incompatibles entre elles : certaines sont ignorées
pdf-warn-finish-before-start = fin lue avant le début lu
pdf-warn-predecessor-unreadable = prédécesseur illisible, ignoré
pdf-warn-predecessor-external = prédécesseur d'un autre projet, ignoré
pdf-warn-predecessor-unknown = prédécesseur absent du tableau, ignoré
pdf-warn-predecessor-self = tâche prédécesseur d'elle-même, ignoré
pdf-warn-level-jump = retrait de plus d'un niveau : ramené au niveau suivant
pdf-warn-stray-lines = lignes de texte du tableau rattachées à aucune tâche
pdf-warn-truncated = bornes de lecture atteintes : contenu partiel
pdf-warn-pages-skipped = pages au-delà de la borne non lues
pdf-warn-link-refused = lien refusé par le moteur (boucle), ignoré
pdf-warn-constraint-refused = contrainte refusée par le moteur
pdf-warn-flattened = hiérarchie non lisible : toutes les tâches au niveau 1
pdf-refused = PDF non importé : { $detail }
pdf-convert-failed = Planning lu, mais le moteur n'a pas pu le construire : { $detail }
pdf-hint-encrypted = Enregistrez depuis votre lecteur PDF une copie sans protection, puis importez-la.
pdf-hint-scanned = Demandez à l'auteur le PDF exporté depuis son logiciel de planning (impression vers PDF), ou ressaisissez le planning.
pdf-hint-no-table = L'import lit les vues Gantt imprimées depuis MS Project ou un logiciel semblable : un tableau avec « Nom de la tâche » et des dates.
pdf-hint-no-dates = Réimprimez le planning avec les colonnes Début et Fin, puis importez-le.
pdf-hint-hierarchy = Cochez « Importer sans hiérarchie » : toutes les tâches au niveau 1, à remettre en retrait ensuite.
pdf-hint-damaged = Le fichier est peut-être abîmé : ouvrez-le dans un lecteur PDF, réimprimez-le en PDF, puis importez la copie.
pdf-hint-too-large = Imprimez seulement les pages du planning dans un PDF plus petit, puis importez-le.
pdf-date-order = Ordre des dates
pdf-order-day-month = Jour/mois (31/12)
pdf-order-month-day = Mois/jour (12/31)
pdf-order-assumed = Aucune date ne tranche entre jour/mois et mois/jour : vérifiez quelques dates, ou imposez l'ordre.
pdf-flatten = Importer sans hiérarchie (toutes les tâches au niveau 1)
pdf-pin-starts = Contraindre les tâches liées à leur début lu
pdf-pin-starts-hint = Pose « Début au plus tôt le » sur les tâches liées que le calcul placerait avant leur début lu (décalage ou contrainte non imprimés dans le PDF).
pdf-drawn-links = Recréer les liens dessinés
pdf-calendar = Calendrier
pdf-calendar-hint = Le PDF n'imprime pas son calendrier : des jours chômés inconnus font finir des tâches plus tôt que dans le PDF.
pdf-holidays = Jours fériés français
pdf-holidays-hint = Les 11 fêtes légales de métropole, de { $from } à { $to } ; ni Alsace-Moselle, ni outre-mer.
pdf-holidays-scope = Les 11 fêtes légales de métropole ; ni Alsace-Moselle, ni outre-mer.
pdf-periods = Périodes chômées déduites des écarts (à cocher) :
pdf-periods-none = Aucune période proposée : les écarts restants ne s'expliquent pas clairement par des jours chômés.
pdf-period-line =
    Du { $from } au { $to } : { $count ->
        [one] { $count } jour ouvré
       *[other] { $count } jours ouvrés
    }, { $n ->
        [one] explique { $n } tâche
       *[other] explique { $n } tâches
    }
pdf-period-place =
    À placer : le PDF dit seulement { $count ->
        [one] { $count } jour chômé
       *[other] { $count } jours chômés
    } entre le { $from } et le { $to }.
pdf-period-earlier = Placer plus tôt
pdf-period-later = Placer plus tard
pdf-period-name = Congés (déduits de l'import PDF)
pdf-unexplained =
    { $count ->
        [one] { $count } tâche garde un écart qu'aucun jour chômé n'explique (demi-journée, décalage) : laissé tel quel.
       *[other] { $count } tâches gardent un écart qu'aucun jour chômé n'explique (demi-journées, décalages) : laissés tels quels.
    }
pdf-same = Dates identiques au PDF : { $n } sur { $count }
pdf-only-gaps = Écarts seulement
pdf-col-id = N°
pdf-col-name = Nom
pdf-col-duration = Durée
pdf-col-start-read = Début lu
pdf-col-finish-read = Fin lue
pdf-col-finish-computed = Fin calculée
pdf-col-gap = Écart
pdf-gap-start = début { $value } j
pdf-gap-finish = fin { $value } j
pdf-no-rows = Aucune tâche avec un écart.
error-pdfimport-too_large = PDF trop volumineux (64 Mio au plus ; 16 Mio dans la version web)
error-pdfimport-encrypted = PDF protégé par un mot de passe ou des droits : non lisible
error-pdfimport-invalid = fichier PDF illisible
error-pdfimport-reader_failure = le lecteur PDF a échoué sur ce fichier
error-pdfimport-no_text = aucun texte lisible dans ce PDF (dessin ou polices sans texte)
error-pdfimport-scanned = PDF numérisé (image) : l'import lit seulement les PDF exportés depuis un logiciel de planning
error-pdfimport-no_table = aucun tableau de planning reconnu (en-tête « Nom de la tâche » avec « Durée », « Début » ou « Fin »)
error-pdfimport-no_dates = le tableau ne contient pas de dates de début ou de fin
error-pdfimport-flat_hierarchy = liste sans retrait avec des titres en gras : la hiérarchie ne peut pas être lue sûrement
error-pdfimport-inconsistent_hierarchy = les retraits des noms ne correspondent pas aux dates des récapitulatives : la hiérarchie ne peut pas être lue sûrement
error-pdfimport-no_tasks = tableau reconnu, mais aucune tâche lue
error-pdfimport-too_many_tasks = plus de tâches que la borne de l'import (20 000)
