# Import d'un planning PDF (lots P1a à P3)

Décisions : ADR-0032 (P1a), ADR-0033 (P1b), ADR-0034 (P1c). Règles de calcul : R-270 à R-280 (`docs/scheduling-rules.md`). Code : `crates/plannercraft-pdfimport`, CLI `plannercraft import-pdf`, fenêtre Fichier → Importer un PDF… (`crates/plannercraft-ui-egui/src/dialog/pdf_import.rs`, `docs/features/ui.md` § 1.53 et § 1.54). Guide : `docs/user/pdf-import.md`.

## 1. Besoin

Un maître d'œuvre ou un OPC joint au dossier de consultation un planning Gantt **imprimé en PDF** depuis MS Project : un tableau à gauche (N°, nom, durée, début, fin, parfois prédécesseurs, ressources), le graphique à droite. L'entreprise veut en faire un planning PlannerCraft modifiable, sans ressaisie.

## 2. Lots

| Lot | Contenu | État |
|---|---|---|
| **P1a** | Moteur d'import et sa mesure, sans interface : lecture du PDF, tableau, valeurs, refus typés, projet construit par les commandes du moteur, rapport, CLI, corpus public | livré (ce document) |
| **P1b** | Interface : Fichier → Importer un PDF…, aperçu du tableau lu, avertissements, réglages (ordre jour/mois, sans hiérarchie, tâches liées), calendrier (jours fériés français, périodes chômées déduites des écarts), ouverture comme document neuf | livré (§ 10) |
| **P1c** | Liens relus dans les flèches du graphique quand le PDF n'a pas de colonne Prédécesseurs, posés par le moteur avec un décalage choisi pour garder les dates lues ; case dans la fenêtre, options de la CLI | livré (§ 11) |
| **P2** | Copie du style : couleurs des barres, format de page, colonnes lues dans le PDF → modèle personnel | à faire (données déjà extraites, § 6) |
| **P3** | Aide par LLM pour les cas refusés en P1a (scan via OCR, fausse hiérarchie, tableau sans dates) : le LLM propose une structure, **jamais une date** (règle 3) ; validation par l'utilisateur | à faire |

## 3. Pipeline (P1a)

Entrée : des octets (aucun système de fichiers ; compile en `wasm32-unknown-unknown`). Sortie : `ImportedPlan` (neutre, dates **lues**), puis `Converted` (session, dates **calculées**, écarts).

1. **Extraction** (`extract.rs`, seul module à flottants) : `hayro-interpret` interprète chaque page dans un appareil de collecte. Glyphes → caractères Unicode (table `ToUnicode`), positions, corps, gras (nom PostScript, graisse ≥ 600, rendu remplir-et-contourer) ; glyphes hors de la découpe de cellule écartés ; texte invisible (OCR) compté. Chemins découpés en sous-chemins : aplats (rectangle ou non, couleur RVB) et traits horizontaux/verticaux. Images : surface couverte. Page imprimée de côté (texte vertical majoritaire) : relue tournée d'un quart de tour. Coordonnées converties une fois en **centièmes de point entiers**.
2. **Mots et lignes** (`extract.rs`, `lines.rs`) : glyphes regroupés en mots (même ligne de base, écart < 0,25 corps dans un même appel de dessin, < 0,03 corps entre deux appels : deux cellules voisines se touchent parfois), lignes par ligne de base (un N° en petit corps et un nom en grand corps restent ensemble).
3. **En-tête et colonnes** (`header.rs`) : première bande (1 ou 2 lignes) du haut de page qui porte un « Nom de la tâche » / « Task Name » et une durée ou une date (dictionnaire FR/EN replié). Bornes : traits verticaux du quadrillage qui partent de l'en-tête ; sinon le plus large vide de la projection horizontale des mots entre deux intitulés. Colonnes non reconnues entre deux traits gardées comme « autre ». Bas du tableau : fin du quadrillage.
4. **Lignes de tâches** (`table.rs`) : une tâche commence à une ligne qui porte un N° (sinon un nom et une durée ou une date) ; les lignes suivantes proches (≤ 1,7 corps) complètent les cellules (noms repliés, prédécesseurs collés sans espace) ; légende « Projet : … » ou « Date : … » : fin du tableau de la page ; mots coupés aux traits de colonne au glyphe près. Retrait du nom mesuré depuis le bord de sa colonne ; symboles de plan (triangles) écartés.
5. **Valeurs** (`values.rs`) : écriture ramenée à la forme canonique puis lue par les analyseurs de `plannercraft-automation` (R-140, R-142) : « 5j », « 22 jours », « 3 jrs », « 2 sm », « 1 sem. », « 14,2 moiséc », « 5 edays », « 3 jours écoulés », « 1 jour? », « 157 days », « 1.5 wks », « 2 mons », « 3 m » (mois dans un document français depuis P1b, minutes en anglais) ; prédécesseurs « 3FD+6 jours;5 », « 13,26FS-11 days », « 12FD+50% » ; dates « Ven 03/04/15 », « Mon 1/21/19 », « lun. 21/01/2019 08:00 », « 1/21/19 5:00 PM », « 21 janv. 2019 », « 21-Jan-19 », « Jan 21 '19 », « 05.04.17 », ISO ; « NA », « tba » : pas de date.
6. **Planning lu** (`plan.rs`) : dédoublonnage par N° (tableau répété, rappel « (suite) » / « (continued) ») ; tri par N° ; lignes vides et N° 0 écartés ; cellules débordantes ramenées à leur valeur typée (texte en trop rendu au nom) ; niveaux : colonne WBS si ≥ 80 % de numéros hiérarchiques, sinon rang du retrait (à 2,5 pt près) ; récapitulative = suivante plus profonde ; ordre jour/mois sur tout le document (R-273) ; confiance par ligne (0 à 100, douteuse sous 70) ; avertissements à code stable (`pdfimport.*`, messages FR/EN).
7. **Conversion** (`convert.rs`) : `Session::new_project` au plus petit début lu (R-270) ; un lot `task.add` (hiérarchie par `last_child_of`, durée lue sur les feuilles, jalon si durée nulle) ; un lot `link.add` (un par un s'il est refusé) ; contraintes SNET (R-271) ; nouvelle session sans historique ; comparaison au jour (R-272).
8. **Rapport** (`report.rs`) : texte lisible et vue JSON stable (tâches, avertissements, colonnes, couleurs, conversion).

## 4. Refus (jamais d'import de travers en silence)

| Code | Cas | Détection |
|---|---|---|
| `pdfimport.too_large` | > 64 Mio | taille |
| `pdfimport.encrypted` / `invalid` | PDF chiffré ou illisible | lecteur |
| `pdfimport.reader_failure` | panique du lecteur interceptée (desktop) | `catch_unwind` |
| `pdfimport.no_text` | aucun texte lisible | < 20 glyphes visibles à table Unicode |
| `pdfimport.scanned` | numérisé (image pleine page, texte absent ou invisible) | surface d'image ≥ 50 %, glyphes invisibles |
| `pdfimport.no_table` | pas d'en-tête de tableau Gantt (autre outil, autre document) | dictionnaire d'intitulés |
| `pdfimport.no_dates` | tableau sans colonne ni valeur de date | colonnes, valeurs |
| `pdfimport.flat_hierarchy` | liste sans retrait avec titres en gras | un seul retrait, gras mêlé |
| `pdfimport.inconsistent_hierarchy` | retraits contredits par les dates (récapitulatives qui ne contiennent pas leurs « enfants ») | ≥ 3 récapitulatives datées, moins de la moitié cohérentes |
| `pdfimport.no_tasks` / `too_many_tasks` | tableau vide / > 20 000 tâches | compte |

`--flatten` (option `allow_flat_hierarchy`) importe quand même les deux cas de hiérarchie, tout au niveau 1 (`pdfimport.flattened`). Bornes : 500 pages, 200 000 glyphes, 50 000 aplats et 50 000 traits gardés par page (`pdfimport.truncated` au-delà).

## 5. Dates (règle 3)

Les dates lues ne sont jamais recopiées comme dates calculées. Elles servent à : (a) fixer le début du projet (R-270) ; (b) poser « Début au plus tôt le » sur une feuille sans prédécesseur que le moteur placerait plus tôt (R-271), comme une saisie dans MS Project ; (c) comparer, au jour (R-272). Option `--pin-starts` (R-271 étendue, à arbitrer par le propriétaire, ADR-0032 D5) : contrainte aussi sur les feuilles liées placées trop tôt, posée à la source de l'écart par passes. La durée lue n'est jamais forcée pour coller aux dates : une durée « 5 j » lue du mercredi au mercredi suivant donne un écart de fin listé.

Écarts observés (déduit) : sans lien, ils portent sur les **fins** et le moteur finit **plus tôt** : jours chômés absents du calendrier Standard (lundi de Pâques et jours fériés de mai : −1 j sur les tâches d'avril-mai ; fin d'année : −7 à −8 j ; été : −10 à −16 j), et début ou fin en milieu de journée. Avec liens, s'y ajoutent les contraintes et décalages non imprimés (Wellesley : 69/366 dates égales par défaut, 359/366 avec `--pin-starts`).

## 6. Ce que la page donne pour P2 (observé)

- **Format** : taille de la page de rendu (A3 paysage 420 × 297 mm sur les plannings du propriétaire et plusieurs fichiers publics ; tabloïd, lettre, A2, A4).
- **Colonnes** : intitulé tel que lu, sorte reconnue, bornes en centièmes de point (`SourceInfo.columns`) ; colonnes inconnues gardées (« Responsable », « Cadence », « Quantité », « Unité »…).
- **Couleurs** : aplats rectangulaires de 3 à 20 pt de haut dans la zone graphique, comptés par couleur (`SourceInfo.bar_colors`) : par exemple `#808080` (62 barres) et deux couleurs d'accent sur un fichier public ; 7 à 13 couleurs distinctes par planning du propriétaire (bleus `#9cc3e5`/`#bdd7ee`, orangés `#ffc000`, rouges `#ff0000`, gris des récapitulatives). Fonds des lignes récapitulatives : aplats colorés dans la zone du tableau (non exploités).
- **Liens** (relevés depuis P1c, § 11) : aucune colonne Prédécesseurs dans les plannings du propriétaire ; les flèches sont des polylignes de traits horizontaux et verticaux (1 700 à 3 700 traits par page, quadrillage compris) terminées par de petites formes pleines (pointes de flèche à trois sommets ; losanges de jalon et bouts de récapitulatives à quatre ou cinq).

## 7. Corpus et mesure

Corpus public : `tests/corpus-pdf/manifest.toml` (31 fichiers, URL, taille, SHA-256 ; `cargo xtask corpus fetch-pdf`, dossier ignoré par git ; jamais redistribués). Test : `cargo test -p plannercraft-pdfimport --test corpus -- --nocapture` (fichier absent ignoré ; `PLANNERCRAFT_PDF_CORPUS=required` exige au moins un fichier ; `PLANNERCRAFT_PDF_SAMPLES=<dossier>` mesure les plannings du propriétaire sous les noms R1…). Mesure du 2026-11-14, build release, « ms » = import + conversion.

Colonnes : **N° max** = plus grand N° lu (tâches attendues si rien n'est replié) ; **Lignes pdftotext** = N° distincts portant une date dans `pdftotext -layout` (oracle de test hors produit) ; **Récap. cohérentes** = récapitulatives dont les dates lues contiennent celles de leurs enfants ; **Liens** = prédécesseurs relus (ignorés : absents du tableau, illisibles, externes) ; **Dates égales** = dates calculées égales aux dates lues au jour, sur les dates comparées ; **Lignes identiques** = lignes pdftotext dont N°, durée, début et fin lus par l'import sont identiques (comparaison automatique, `pdftotext` comme oracle).

### Corpus public, importés

| Fichier | Pages | N° max | Tâches lues | Lignes pdftotext | Niveaux | Récap. cohérentes | Liens (ignorés) | Débuts · fins relus | Douteuses | Dates égales R-271 | Dates égales `--pin-starts` | Lignes identiques à pdftotext | ms |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| [en-addison-tx-wo1](https://agendas.addisontx.gov/docs/2020/CM/20200512_6714/3525_ADS-Work%20Order%20No.%201-Project%20Schedule%20Summary%202019-09-25.pdf) | 1 | 28 | 8 | 8 | 2 | 1/1 | 0 (0) | 8/8 · 8/8 | 0 | 9/16 | 9/16 | 8/8 | 3 |
| [en-buffalo-sewer-nfa](https://buffalosewer.org/app/uploads/2023/08/Attachment-A-NFA-Schedule.pdf) | 7 | 443 | 317 | 301 | 5 | 49/50 | 0 (0) | 317/317 · 317/317 | 0 | 624/634 | 624/634 | 298/301 | 58 |
| [en-coquille-kahp](https://www.coquilletribe.org/wp-content/uploads/2026/06/BOC-06-04-26-9-ED-Att.-3A-Partney-KAHP-Construction-Schedule-05-07-25.pdf) | 2 | 79 | 79 | 79 | 4 | 15/15 | 64 (0) | 79/79 · 79/79 | 0 | 158/158 | 158/158 | 79/79 | 11 |
| [en-crissair-phased](https://www.crissair.com/documents/Crissair-Canyon-Phased-Milestone.pdf) | 3 | 89 | 30 | 30 | 2 | 3/3 | 9 (3) | 30/30 · 30/30 | 0 | 51/60 | 60/60 | 30/30 | 48 |
| [en-csuohio-lab](https://www.csuohio.edu/sites/default/files/SI%20Instrumentation%20Lab-Schedule_OCT.pdf) | 2 | 31 | 31 | 31 | 3 | 5/5 | 0 (0) | 31/31 · 31/31 | 0 | 55/62 | 55/62 | 31/31 | 15 |
| [en-geneva-oh-atb](https://genevaohio.gov/wp-content/uploads/2024/09/PID-111486-SCHEDULE-9-13.pdf) | 4 | 133 | 127 | 127 | 5 | 12/15 | 97 (0) | 127/127 · 127/127 | 0 | 52/254 | 82/254 | 127/127 | 44 |
| [en-icann-program-tool](https://gnso.icann.org/sites/default/files/file/field-file-attach/program-management-tool-21oct20-en.pdf) | 6 | 334 | 320 | 320 | 6 WBS | 81/81 | 134 (0) | 320/320 · 320/320 | 0 | 613/640 | 640/640 | 373/373 | 105 |
| [en-lsbrd-clubhouse](https://www.lsbrd.org/files/4f74adcfd/LSB+Clubhouse+Milestone+Bid+Schedule+4-1-23.pdf) | 12 | 30 | 30 | 29 | 3 | 4/4 | 25 (0) | 30/30 · 30/30 | 0 | 45/60 | 45/60 | 29/29 | 10 |
| [en-psu-thesis-schedule](https://www.engr.psu.edu/ae/thesis/portfolios/2008/cam459/TECH%20TWO/pdf%20files%20for%20posting/Mowery_Casey_Tech2%20Detailed%20Project%20Schedule.pdf) | 4 | 200 | 186 | 186 | 3 | 17/17 | 0 (0) | 186/186 · 186/186 | 0 | 367/372 | 367/372 | 186/186 | 125 |
| [en-salem-nh-schedule](https://salemnh.gov/DocumentCenter/View/267/Project-Schedule-Revised-01-22-2016-PDF) | 3 | 123 | 123 | 123 | 4 | 23/23 | 0 (0) | 123/123 · 123/123 | 0 | 224/246 | 224/246 | 123/123 | 116 |
| [en-wellesley-hardy-sd](https://wellesleyma.gov/DocumentCenter/View/25969/41219D1-Wellesley-Hardy-Project-Schedule-210707-SD-Phase) | 4 | 216 | 185 | 183 | 5 | 19/19 | 180 (10) | 183/185 · 183/185 | 0 | 69/366 | 359/366 | 181/181 | 30 |
| [en-wellesley-hardy-upham](https://wellesleyma.gov/DocumentCenter/View/21955/SECTION-41110-Wellesley-Hardy-Upham-Project-Schedule) | 2 | 168 | 118 | 117 | 4 | 13/13 | 115 (1) | 118/118 · 118/118 | 0 | 27/236 | 236/236 | 116/116 | 19 |
| [en-westminster-wharncliffe-ac103](https://www.westminster.gov.uk/media/document/_7-06-2022-wharncliffe-gardens-repairs-and-decorations-project-ac103---provisional-programme-of-works-june-2022) | 11 | 640 | 272 | 272 | 5 | 31/31 | 0 (0) | 272/272 · 272/272 | 0 | 489/544 | 489/544 | 272/272 | 59 |
| [en-westminster-wharncliffe-exit](https://www.westminster.gov.uk/media/document/wharncliffe-exit-programme-140923) | 9 | 110 | 109 | 103 | 3 | 8/8 | 0 (0) | 105/109 · 105/109 | 0 | 140/210 | 140/210 | 101/101 | 25 |
| [fr-cern-bat311](https://indico.cern.ch/event/536498/contributions/2194842/attachments/1285781/1912314/Planning_Execution_Travaux_genie_civil_CERN_Bat_311_ind_B.pdf) | 1 | 160 | 75 | 74 | 4 | 11/11 | 65 (1) | 75/75 · 75/75 | 0 | 11/150 | 34/150 | 72/72 | 14 |
| [fr-dreal-paca-amenagement](https://www.paca.developpement-durable.gouv.fr/IMG/pdf/f09324p0120_planning.pdf) | 1 | 106 | 87 | 87 | 6 | 22/22 | 0 (0) | 87/87 · 87/87 | 0 | 167/174 | 167/174 | 87/87 | 12 |
| [fr-itu-council-schedule](https://www.itu.int/en/council/2020/Documents/007/INF%20E_Schedule.pdf) | 1 | 77 | 77 | 77 | 4 | 13/13 | 0 (0) | 77/77 · 77/77 | 0 | 119/154 | 119/154 | 77/77 | 12 |
| [fr-mios-ecole-ramonet](https://www.villemios.fr/wp-content/uploads/2011/02/Calendrier-previsionnel.pdf) | 3 | 13 | 13 | 13 | 1 | 0/0 | 0 (0) | 13/13 · 13/13 | 0 | 20/26 | 20/26 | 39/39 | 21 |
| [fr-villequiers-mairie](https://archilio.fr/wp-content/uploads/2018/03/planning-chantier-mairie-villequiers-5.pdf) | 1 | 43 | 43 | 43 | 1 | 0/0 | 0 (0) | 43/43 · 43/43 | 0 | 84/86 | 84/86 | 43/43 | 9 |
| [mixed-en-coopercity-memo-schedule](https://mccmeetingspublic.blob.core.usgovcloudapi.net/coopercity-meet-2fcdc0818d7748d6ba2112c27ce83800/ITEM-Attachment-001-b0dbd45a4f344a1bb7b08c2b15c63f64.pdf) | 4 | 52 | 17 | 17 | 3 | 3/3 | 5 (6) | 17/17 · 17/17 | 0 | 27/34 | 29/34 | 17/17 | 17 |
| [mixed-en-tdhca-pebble-hills-rotated](https://tdhca.legistar.com/View.ashx?GUID=AC4CBAF5-5659-493A-873A-11494F733D14&ID=14910338&M=F) | 3 | 48 | 48 | 49 | 1 | 0/0 | 0 (0) | 48/48 · 48/48 | 0 | 96/96 | 96/96 | 48/48 | 15 |
| [mixed-en-warrington-wwl](https://www.warrington.gov.uk/sites/default/files/2019-09/outline_business_case_-_appendix_n_-_delivery_programme.pdf) | 9 | 443 | 433 | 432 | 6 | 110/114 | 382 (0) | 433/433 · 433/433 | 0 | 748/866 | 748/866 | 428/432 | 73 |

Écarts relevés par l'oracle : Buffalo (3) et Warrington (4) — tableaux sans quadrillage où la fin d'un nom touche la durée (« Phase 1 » + « 20 days » lus « 120 days ») ; Mios : tableau répété sur 3 pages (39 lignes pdftotext pour 13 tâches, dédoublonnées). Lignes pdftotext supérieures aux tâches lues : lignes vides ou datées hors tableau ; inférieures : lignes sans date (« tba ») ou noms sur plusieurs lignes.

### Corpus public, refusés

| Fichier | Refus attendu | Obtenu |
|---|---|---|
| [en-cumberland-pa-plan-schedule](https://cumberlandcountypa.gov/DocumentCenter/View/44861/Project-Schedule) | pdfimport.no_dates | pdfimport.no_dates |
| [en-health-scope-annexure-b](https://a206977a.delivery.rocketcdn.me/wp-content/uploads/2026/05/Part-C3-Scope-of-Works-Annexure-B-Clients-Activities-1.pdf) | pdfimport.no_dates | pdfimport.no_dates |
| [en-herefordshire-plan](https://councillors.herefordshire.gov.uk/documents/s50013890/Draft%20Project%20Plan%2015%207%2013.pdf) | pdfimport.flat_hierarchy | pdfimport.flat_hierarchy |
| [en-psu-2007-appendix-a](https://www.engr.psu.edu/ae/Thesis/portfolios/2007/DFF110/Appendix%20A%20Detailed%20Schedule.pdf) | pdfimport.inconsistent_hierarchy | pdfimport.inconsistent_hierarchy |
| [fr-beychac-dce](https://www.beychac-cailleau.fr/wp-content/uploads/2017/01/Planning-Previsionnel-DCE-V2.pdf) | pdfimport.no_dates | pdfimport.no_dates |
| [fr-cdesages-gros-oeuvre](https://www.cdesages.com/fichs/23622.pdf) | pdfimport.no_dates | pdfimport.no_dates |
| [fr-indre-master-schedule-t3s](https://www.indre.gouv.fr/contenu/telechargement/35342/309645/file/Annexe%203_Plannif%20Master%20schedule%20T3S%2020210319%20sans%20montant.pdf) | pdfimport.no_dates | pdfimport.no_dates |
| [other-en-p6-crossville-detailed](https://crossvilletn.gov/wp-content/uploads/2024/10/Detailed-Schedule-October-72024.pdf) | pdfimport.no_table | pdfimport.no_table |
| [other-en-p6-hse-pacific-village](https://www.hsecontractors.com/wp-content/uploads/2023/11/Summary-Schedule-Pacific-Village-CA.pdf) | pdfimport.no_table | pdfimport.no_table |

Cumberland, Health, Beychac, Cdesages, Indre : tableaux sans dates (P3). Herefordshire : liste plate, la ligne 1 en gras couvre tout le projet. PSU 2007 : feuilles au bord, récapitulatives en retrait (retraits trompeurs, P3). Primavera P6 : autre outil.

### Plannings du propriétaire (R1 à R6)

Imprimés par une imprimante PDF virtuelle (Ghostscript), A3 paysage, 1 ou 2 pages, colonnes N°, Nom, Durée, Début, Fin (parfois Cadence, Quantité, Unité), sans prédécesseurs, vues filtrées (N° non continus, gardés tels quels).

| Planning | Pages | N° max | Tâches lues | Lignes pdftotext | Niveaux | Récap. cohérentes | Liens (ignorés) | Débuts · fins relus | Douteuses | Dates égales R-271 | Dates égales `--pin-starts` | Lignes identiques à pdftotext | ms |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| R1 | 2 | 187 | 168 | 168 | 8 | 39/39 | 0 (0) | 168/168 · 167/168 | 0 | 281/335 | 281/335 | 166/168 | 28 |
| R2 | 1 | 207 | 91 | 91 | 7 | 22/22 | 0 (0) | 91/91 · 91/91 | 0 | 132/182 | 132/182 | 90/91 | 15 |
| R3 | 2 | 248 | 219 | 219 | 5 | 40/40 | 0 (0) | 219/219 · 219/219 | 0 | 382/438 | 382/438 | 219/219 | 33 |
| R4 | 1 | 266 | 128 | 128 | 6 | 34/34 | 0 (0) | 128/128 · 128/128 | 0 | 174/256 | 174/256 | 127/127 | 19 |
| R5 | 1 | 67 | 62 | 62 | 5 | 12/12 | 0 (0) | 62/62 · 62/62 | 0 | 89/124 | 89/124 | 62/62 | 13 |
| R6 | 1 | 216 | 108 | 108 | 5 | 19/19 | 0 (0) | 108/108 · 108/108 | 0 | 191/216 | 191/216 | 108/108 | 15 |

Vérifié à la main (R1 et R4, 8 lignes chacun réparties sur tout le tableau) : N°, niveau (comparé au retrait du nom dans `pdftotext -layout`), durée, début, fin ; tous conformes. Lignes non identiques à l'oracle : R1 N° 78, fin « 20/09/28 » dont le dernier glyphe est hors de la découpe de la cellule (écarté par l'import, lu par pdftotext : `pdfimport.date_unreadable`) ; R1 et R2, deux lignes où l'oracle prend pour une date un numéro du nom (artefact de l'oracle, l'import est juste). Public vérifié à la main : CERN (75 lignes relues contre pdftotext : niveaux, durées « sm » et « moiséc », liens FD/DD/DF/FF avec décalages), Geneva et PSU 2007 (retraits).

Temps : 125 ms au plus par fichier en release (PSU thesis, 4 pages, 186 tâches) ; le plus gros fichier du corpus (P6, 1,45 Mo, 11 pages) est refusé en ~3 s en debug.

## 8. Limites connues

- Calendrier, jours fériés, heures de début et de fin, contraintes autres que la date de début, nivellement : invisibles dans le PDF ; seulement révélés par les écarts. Depuis P1b : fériés français proposés et périodes chômées déduites des écarts, à valider (§ 10).
- Liens absents du tableau (plannings du propriétaire) : relevés dans le graphique depuis P1c (§ 11), avec les limites d'ADR-0034.
- Tableaux sans quadrillage : un nom qui touche la durée peut lui céder un chiffre (7 lignes sur 733 dans deux fichiers).
- Glyphe à moitié coupé par la cellule : écarté (une date tronquée sur 776 lignes du propriétaire).
- Pages lues dans l'ordre ; tableau sans colonne N° : pas de dédoublonnage par N°.
- Hiérarchie lue par le retrait ou la colonne WBS seulement (pas par le gras seul) ; ressources et % achevé lus mais non importés.
- Panique du lecteur interceptée sur le desktop seulement (en WebAssembly, une panique interrompt le module).
- Scans réels : aucun dans le corpus public (refus testé sur des pages synthétiques seulement).

## 9. Recommandations

- **P1b** (livré, § 10 ; ce qui suit est la recommandation d'origine) : aperçu tableau (N° lu, niveau, nom, durée, début lu, fin lue, confiance) avec filtres « douteuses » et « écarts » ; réglages exposés : ordre jour/mois, aplatir, `--pin-starts` (par défaut à trancher, ADR-0032 D5) ; ouverture via `document::load_project(converted.session.project())` comme l'import CSV ; rapport dans le panneau des diagnostics. Avant P1b, proposer un « calendrier déduit » : jours chômés qui expliquent tous les écarts de fin (calcul du moteur, déterministe), à faire valider.
- **P2** : `SourceInfo` donne format, colonnes et couleurs ; associer chaque couleur à un rôle de barre (récapitulative = barre la plus haute alignée sur une ligne récapitulative, jalon = forme non rectangulaire) avant d'écrire un modèle.
- **Liens dessinés** : livré en P1c (§ 11) ; suites possibles : recoudre les liens qui passent un saut de page, lire les jonctions en T.

## 10. Lot P1b : interface et calendrier (2026-11-14)

### 10.1 Fenêtre d'import

Fichier → Importer un PDF… (aussi dans la palette de commandes, aussi sans document ouvert) : sélecteur limité aux `.pdf`, octets lus localement (rien n'est envoyé, sur le web non plus), puis fenêtre d'aperçu (H-UI-205 à H-UI-212, `docs/features/ui.md` § 1.53) :

- **résumé** : tâches lues, niveaux, liens, colonnes reconnues (intitulés lus), avertissements du moteur traduits (six au plus, puis « … et n autres ») ;
- **réglages** montrés quand ils servent : ordre jour/mois (si aucune date ne le tranche), « importer sans hiérarchie » (après un refus pour hiérarchie plate ou incohérente, au lieu d'un refus sec), contrainte des tâches liées (`pin_starts`, si le PDF a des liens, décochée, avec une phrase qui dit ce qu'elle fait) ;
- **calendrier** : case « Jours fériés français » (cochée si l'en-tête est français), périodes chômées déduites des écarts (décochées), compteur « Dates identiques au PDF : n sur m » ;
- **tableau** des tâches lues, virtualisé : N°, nom en retrait par niveau, durée lue, début lu, fin lue, fin calculée, écart (en couleur), filtre « Écarts seulement » ;
- **Importer** ouvre le projet comme un nouveau document non enregistré nommé d'après le fichier (confirmation si le document courant est modifié) ; **Annuler** (ou Échap) ne touche à rien ; un PDF refusé affiche le motif et, quand c'est utile, quoi faire.

Calcul (lecture, conversion, déduction) dans un fil dédié sur le bureau, un seul à la fois ; sur le web, à l'image qui suit « Lecture du PDF en cours… ». Web : 16 Mio et 50 pages au plus (64 Mio et 500 pages sur le bureau) ; une panique du lecteur n'y est pas interceptable : la page s'arrête, la fenêtre le dit (ADR-0033 D3).

### 10.2 Jours fériés français (R-274, R-275)

Les onze fêtes légales de métropole (article L3133-1 du Code du travail), Pâques calculé en entiers (`plannercraft_core::calendar::holidays`), pour chaque année couverte par les dates lues et la suivante, posées comme exceptions « chômé » du calendrier du projet (`calendar.set_exception`) avant les tâches. Ni Alsace-Moselle (Vendredi saint, 26 décembre) ni outre-mer.

### 10.3 Calendrier déduit des écarts (R-276, R-277)

Algorithme retenu (entier, déterministe ; la piste du cahier, complétée sur les faits) :

1. sur le calendrier `C` (Standard plus fériés cochés), chaque feuille à durée entière en jours, dates lues aux heures par défaut, donne `W` = jours ouvrés de `C` dans `[début lu, fin lue]` et `d` jours de durée : `W = d` prouve ces jours travaillés, `W > d` dit que `W − d` d'entre eux ont été chômés, `W < d` (ou une date lue un jour chômé de `C`) contredit `C` ;
2. jours candidats : ouvrés, couverts par une équation, par aucune preuve ; une équation qui a moins de candidats que de jours à retirer est impossible (demi-journée, décalage) et n'est pas forcée ;
3. les candidats forment des suites de jours ouvrés consécutifs ; dans chaque suite, toutes les périodes continues de 20 jours ouvrés au plus sont évaluées : aucune équation dépassée, le plus d'équations locales satisfaites exactement, majorité stricte des équations locales de la suite ;
4. la plus courte des meilleures périodes, unique et contenue dans toutes les autres, est **déterminée** ; sinon les plus courtes sont des **placements** équivalents (« à placer » : le PDF dit combien de jours et dans quelle fenêtre, pas lesquels), proposés à partir de 2 jours ouvrés, placement par défaut par la plus longue fermeture avec les week-ends et fériés voisins, puis le plus de fériés, puis le milieu de la fenêtre ; l'utilisateur le déplace (◀ ▶) ;
5. ensemble, les périodes ne dépassent aucune équation ; une période qui n'explique rien, ou d'un seul jour qui explique moins de deux tâches, n'est pas proposée.

Bornes : 20 jours ouvrés par période, 7 400 jours d'étendue (plus deux semaines de marge de chaque côté), 20 000 000 d'évaluations (période × équation) ; au-delà, déduction partielle signalée (`bounded`). Temps mesuré (release, déduction seule) : 0,1 à 0,4 ms par planning du propriétaire, 2,1 ms au plus sur le corpus public.

Constat qui a changé la piste : une période d'été est presque toujours enjambée par **une seule** tâche longue, sans tâche prouvée dans la fenêtre ; la mesure dit alors « 10 ou 15 jours chômés entre tel et tel jour », jamais lesquels. Sans les placements « à placer », aucune période n'était proposée sur cinq des six plannings du propriétaire.

### 10.4 Mesure

Dates calculées égales aux dates lues, au jour (débuts et fins, R-272), sans `--pin-starts`, placements par défaut des périodes, mesure du 2026-11-14 (`cargo test --release -p plannercraft-pdfimport --test corpus pdf_corpus_calendar -- --nocapture`, `PLANNERCRAFT_PDF_SAMPLES` pour les plannings du propriétaire). Quatre cas : calendrier Standard (P1a, après le correctif « 3 m » = 3 mois qui fait gagner 4 dates à R5), fériés seuls, périodes déduites seules (sur le Standard), fériés puis périodes déduites sur ce calendrier. « Périodes proposées » : réglage par défaut de la fenêtre (fériés cochés si l'en-tête est français), mois du placement par défaut, jours ouvrés, tâches expliquées.

| Planning | Standard | Fériés | Périodes seules (n) | Fériés + périodes (n) | Périodes proposées (réglage par défaut) |
|---|---|---|---|---|---|
| R1 | 281/335 | 308/335 | 296/335 (6) | 322/335 (4) | déc. 4 j à placer (2 t.), août 10 j à placer (1 t.), déc. 5 j à placer (2 t.), août 10 j à placer (1 t.) |
| R2 | 132/182 | 156/182 | 138/182 (2) | 165/182 (3) | déc. 4 j à placer (2 t.), août 10 j à placer (3 t.), août 10 j à placer (1 t.) |
| R3 | 382/438 | 405/438 | 400/438 (1) | 425/438 (1) | déc. 4 j à placer (18 t.) |
| R4 | 174/256 | 189/256 | 185/256 (3) | 204/256 (4) | mars 2 j (5 t.), avr. 2 j à placer (2 t.), juin 2 j (2 t.), juin 3 j (3 t.) |
| R5 | 93/124 | 106/124 | 98/124 (3) | 112/124 (2) | août 15 j à placer (1 t.), déc. 4 j à placer (5 t.) |
| R6 | 191/216 | 201/216 | 196/216 (5) | 213/216 (2) | août 15 j à placer (7 t.), déc. 5 j à placer (6 t.) |
| **Total** | **1 253/1 551 (80,8 %)** | **1 365 (88,0 %)** | **1 313 (84,7 %)** | **1 441 (92,9 %)** | |

Plausibilité (déduit, sans fichier source pour le vérifier) : fin d'année, 4 à 5 jours ouvrés placés du 27 ou 28 au 31 décembre (les 25 décembre et 1er janvier étant des fériés) ; été, 2 ou 3 semaines autour du 15 août ; R4 propose des jours isolés (mars, juin) et compte 51 équations pour 30 preuves : son calendrier d'origine n'est sans doute pas le Standard plus des congés (jours d'intempéries, autre semaine type), rien n'est proposé en masse. Restent 110 dates différentes : demi-journées ou décalages (19 équations impossibles), deuxième congé d'une même suite, tâches liées.

Corpus public (22 PDF importés) :

| Fichier | Langue | Standard | Fériés | Périodes seules (n) | Fériés + périodes (n) | Périodes proposées (réglage par défaut) |
|---|---|---|---|---|---|---|
| en-addison-tx-wo1 | en | 9/16 | 9/16 | 9/16 (0) | 9/16 (0) | |
| en-buffalo-sewer-nfa | en | 624/634 | 421/634 | 624/634 (0) | 421/634 (0) | |
| en-coquille-kahp | en | 158/158 | 6/158 | 158/158 (0) | 6/158 (0) | |
| en-crissair-phased | en | 51/60 | 32/60 | 51/60 (0) | 32/60 (0) | |
| en-csuohio-lab | en | 55/62 | 45/62 | 55/62 (0) | 45/62 (0) | |
| en-geneva-oh-atb | en | 52/254 | 53/254 | 52/254 (4) | 53/254 (6) | juin 8 j (2 t.), avr. 2 j (2 t.), juil. 2 j (3 t.), sept. 2 j (2 t.) |
| en-icann-program-tool | en | 613/640 | 292/640 | 613/640 (0) | 292/640 (0) | |
| en-lsbrd-clubhouse | en | 45/60 | 18/60 | 45/60 (1) | 18/60 (1) | sept. 5 j à placer (1 t.) |
| en-psu-thesis-schedule | en | 367/372 | 301/372 | 367/372 (0) | 301/372 (0) | |
| en-salem-nh-schedule | en | 224/246 | 216/246 | 224/246 (0) | 216/246 (0) | |
| en-wellesley-hardy-sd | en | 69/366 | 18/366 | 69/366 (0) | 18/366 (0) | |
| en-wellesley-hardy-upham | en | 27/236 | 4/236 | 27/236 (0) | 4/236 (0) | |
| en-westminster-wharncliffe-ac103 | en | 489/544 | 438/544 | 514/544 (5) | 439/544 (1) | janv. 7 j à placer (1 t.), avr. 2 j à placer (1 t.), juin 2 j à placer (12 t.), déc. 6 j à placer (4 t.), mai 3 j à placer (4 t.) |
| en-westminster-wharncliffe-exit | en | 140/210 | 141/210 | 150/210 (3) | 141/210 (1) | mars 1 j (3 t.), avr. 1 j (9 t.), déc. 10 j (5 t.) |
| fr-cern-bat311 | fr | 11/150 | 87/150 | 11/150 (1) | 87/150 (0) | |
| fr-dreal-paca-amenagement | fr | 167/174 | 149/174 | 172/174 (3) | 153/174 (2) | nov. 5 j à placer (1 t.), oct. 2 j à placer (1 t.) |
| fr-itu-council-schedule | fr | 119/154 | 120/154 | 116/154 (2) | 122/154 (5) | juil. 8 j à placer (1 t.), oct. 2 j (2 t.), juil. 9 j à placer (2 t.), sept. 2 j à placer (1 t.), déc. 4 j (2 t.) |
| fr-mios-ecole-ramonet | fr | 20/26 | 19/26 | 21/26 (1) | 20/26 (1) | janv.-févr. 5 j à placer (1 t.) |
| fr-villequiers-mairie | fr | 84/86 | 80/86 | 85/86 (1) | 81/86 (1) | déc. 8 j à placer (1 t.) |
| mixed-en-coopercity-memo-schedule | en | 27/34 | 17/34 | 27/34 (0) | 17/34 (0) | |
| mixed-en-tdhca-pebble-hills-rotated | en | 96/96 | 59/96 | 96/96 (0) | 59/96 (0) | |
| mixed-en-warrington-wwl | en | 748/866 | 44/866 | 748/866 (0) | 44/866 (0) | |

Totaux : en-tête français (5 fichiers, fériés proposés cochés), 401 → 455 → 405 → 463 sur 590 (68,0 → 77,1 → 68,6 → 78,5 %) ; en-tête anglais (17 fichiers, fériés proposés décochés), Standard 3 794 et périodes seules 3 829 sur 4 854 (78,2 → 78,9 %) ; les fériés français y sont faux (43,6 %), d'où la case décochée. Le CERN gagne 76 dates avec les fériés ; trois PDF français publics en perdent (planificateurs qui ne chômaient pas les fériés) : le compteur le montre, la case se décoche. Sur le planning anglais AC103, sans liste de fériés, la déduction retrouve Pâques 2022 et deux jours dans la semaine du jubilé de juin (placés un jour trop tard par défaut, ADR-0033 D6). Les deux corpus tirent dans le même sens pour les périodes ; pour les fériés, priorité aux fichiers du propriétaire (cochés d'office si l'en-tête est français).

Taille du web (ADR-0033 D3) : `.wasm` gzip -9 +1,00 Mo (mesure locale avant `wasm-opt`), budget C-REL-05 de 4 Mio dépassé. **Arbitré le 2026-11-14** : mesure réelle sur le paquet web de la CI, 13 035 433 octets, 5 332 421 gzip -9 ; l'import PDF reste dans la version web (c'est l'entrée publique voulue par le propriétaire) et le budget passe à 6 Mio.

### 10.5 Limites

Voir ADR-0033 (limites). Pas d'Alsace-Moselle ni d'outre-mer ; une période par suite de jours candidats ; feuilles à durée entière en jours seulement ; aucune vérification dans un navigateur réel ni contre MS Project ; la liste des fériés ne sert pas encore à un planning neuf (aucune entrée d'interface hors de l'import).

## 11. Lot P1c : liens relus dans le dessin (2026-11-14)

Décision : ADR-0034 ; règles R-278 à R-280 ; interface `docs/features/ui.md` § 1.54 (H-UI-213 à H-UI-215) ; CLI `--no-drawn-links`, `--holidays`, `--periods` (`docs/features/cli.md`).

### 11.1 Algorithme retenu

Module `links.rs`, entiers seulement (centièmes de point), après une extraction enrichie (`extract.rs` : couleur, épaisseur, pointillé, rang de sous-chemin et contour fermé de chaque trait ; corde des courbes ; sommets des petites formes pleines) :

1. **Pointes** : formes pleines à trois sommets (0,8 à 15 pt de côté) dont deux sont alignés et le troisième au-delà, dans la zone graphique (à droite de la dernière colonne du tableau, sous l'en-tête) ; sens = côté de la pointe ; pointes superposées au même endroit fusionnées (une par lien qui y arrive, rangs de sous-chemin gardés).
2. **Traits de lien** : traits pleins, de 2 pt au plus, de la couleur d'une pointe de la page (à 4 unités près par composante), sans monter dans l'en-tête de l'échelle de temps ; écartés : côtés de barres, contours fermés (losanges, barres et pointes dessinés au trait), tracé interne des pointes, traits de longueur nulle.
3. **Remontée** depuis chaque pointe : trait qui y arrive (de préférence le dernier dessiné avant la pointe, rang de sous-chemin voisin), puis d'extrémité en extrémité (écart de 0,08 pt), sans revenir sur un trait superposé, sans repasser par un trait plus court que l'écart, la suite de la même polyligne d'abord ; arrêt sur la pointe d'un autre lien atteinte dans son sens (pointe verticale toujours, horizontale seulement par son dernier trait). Un bout libre est un **départ** ; un bout sur le corps d'un autre trait (jonction en T, croisement) est écarté.
4. **Rattachement et type** : tâche = ligne du tableau de même ordonnée (bandes à mi-chemin des lignes voisines, lignes répétées sur plusieurs pages comprises) ; pointe vers la droite, le bas ou le haut = début du successeur, vers la gauche = fin ; départ vers la droite = fin du prédécesseur, vers la gauche = début ⇒ FD, DD, FF, DF ; départ vertical : sous une barre ou un jalon, côté deviné (confiance 65, jamais posé), ailleurs écarté (`cut`). Chaque bout est contrôlé contre les barres de sa ligne (aplats de 1 à 25 pt de haut, formes, côtés verticaux courts des barres étroites ou dessinées au trait) : bout hors barre, confiance 90.
5. **Bilan** : liens triés, dédoublonnés par couple (le plus confiant gardé), écartés comptés par motif (`no_line` forme sans trait, `no_source`, `junction`, `cut`, `source_outside`, `target_outside`, `self_link`, `duplicate`, `bounded`), positions des 200 premières pointes écartées.

Bornes : 20 000 traits candidats et 5 000 pointes par page, 20 000 000 de pas par document (relevé partiel signalé au-delà), 20 000 formes extraites par page. Déterministe (aucune table de hachage, ordre des pages et des tracés).

### 11.2 Vérité terrain publique

PDF publics qui ont à la fois une colonne Prédécesseurs (complétée par la colonne Successeurs quand il y en a une : cellules coupées à l'impression) et le dessin. Mesure du 2026-11-14 (`cargo test --release -p plannercraft-pdfimport --test corpus pdf_corpus_links -- --nocapture`). **Justes** : liens relus présents dans la colonne avec le même type ; **visibles** : liens de la colonne entre deux tâches de la même page qui ont chacune une barre dessinée (un lien entre tâches hors de l'échelle de temps imprimée ne peut pas être dessiné).

| Fichier | Pointes | Liens relus (douteux) | Justes avec le type | Couples justes | Colonne | Visibles | Retrouvés (visibles) | Écartés par motif |
|---|---|---|---|---|---|---|---|---|
| en-coquille-kahp | 64 | 61 (0) | 61 | 61 | 64 | 62 | 61 | junction 1, cut 2 |
| en-crissair-phased | 9 | 1 (0) | 0 | 1 | 9 | 8 | 1 | no_line 2, no_source 1, self_link 5, duplicate 1 |
| en-geneva-oh-atb | 97 | 94 (1) | 93 | 93 | 97 | 93 | 93 | self_link 3 |
| en-icann-program-tool | 132 | 128 (0) | 128 | 128 | 134 | 130 | 128 | junction 2, cut 4 |
| en-lsbrd-clubhouse | 2 | 2 (0) | 2 | 2 | 25 | 2 | 2 | — |
| en-wellesley-hardy-sd | 111 | 101 (0) | 100 | 101 | 188 | 103 | 101 | cut 14 |
| en-wellesley-hardy-upham | 99 | 105 (1) | 103 | 103 | 124 | 116 | 103 | cut 3 |
| mixed-en-coopercity-memo-schedule | 5 | 5 (0) | 5 | 5 | 5 | 5 | 5 | — |
| mixed-en-warrington-wwl | 351 | 310 (0) | 310 | 310 | 382 | 321 | 310 | junction 6, cut 56 |
| fr-cern-bat311 | 0 | 0 | — | — | 65 | — | 0 | liens non dessinés à l'impression |
| **Total** | **870** | **807 (2)** | **802 (99,4 %)** | **804 (99,6 %)** | **1 093** | **905** | **804 (88,8 % des visibles, 73,6 % de la colonne)** | |

Erreurs (observé) : 2 liens douteux (départ vertical en haut de page, côté deviné : jamais posés) ; 2 types faux (DD lu pour un FD, FF lu pour un DF dont les tracés se superposent) ; 1 lien absent de la colonne mais vérifié juste à l'œil (cellule Prédécesseurs coupée à l'impression, « 55FS+ » visible). Parmi les 805 liens posés : 803 justes. Liens visibles manqués : surtout coupés à un saut de page (`cut`, dont 56 sur Warrington) et jonctions. Liens non dessinés : CERN (vue imprimée sans liens), Lake Shore (2 liens visibles sur 25 dans la colonne), Wellesley SD (tâches de 2019 hors de l'échelle de temps imprimée).

Sans colonne (relevé seul, aucune vérité) : AC103 267 pointes → 246 liens, Exit 85 → 73, DREAL 58 → 58, Villequiers 35 → 33, CSU 20 → 0 (19 coupés), PSU 2 → 0, Mios 1 → 0 ; Salem 108 → 0 (Gantt dense : 37 formes sans trait, 32 coupés, 23 bouts dans la ligne de la pointe, 15 sans départ) : rien de posé plutôt que du faux.

### 11.3 Plannings du propriétaire (R1 à R6)

| Planning | Pointes | Liens relus (douteux) | Écartés par motif | Liens posés (refusés) | Décalages négatifs | Contraintes (R-279) | ms lecture (sans relevé) |
|---|---|---|---|---|---|---|---|
| R1 | 124 | 112 (3) | junction 5, cut 6, self_link 1 | 109 (0) | 1 | 36 | 29 (26) |
| R2 | 0 | 0 | — | — | — | — | 14 (14) |
| R3 | 166 | 178 (1) | junction 4, cut 12, self_link 4 | 176 (1) | 9 | 71 | 40 (29) |
| R4 | 0 | 0 | — | — | — | — | 18 (18) |
| R5 | 48 | 48 (1) | — | 47 (0) | 1 | 20 | 13 (12) |
| R6 | 0 | 0 | — | — | — | — | 14 (14) |

R2, R4, R6 : vues imprimées sans liens (aucune pointe). R3 : plus de liens que de pointes (déduit) : des tracés de liens différents se superposent jusqu'au bord d'une barre (un DD qui arrive au début d'une tâche, un autre qui en part ; un FD et un DD qui partagent la descente vers un même successeur) : chaque départ atteint donne un lien, sans fichier source pour trancher.

**Contrôle à l'œil** (pages rendues par `pdftoppm`, hors produit) : R1, 24 liens (jalon de départ vers trois tâches, chaîne de 17 à 31, trois FF vers 13, 14, 15, un DF de 14 vers 12, trois DD partant d'un jalon) ; R5, 13 liens (jalon vers 3, 4, 5, DD de 3 vers 8, FF de 3 vers 15, DF de 15 vers 14, lien qui remonte de 64 vers le jalon 44, chaîne 8 à 12). **37 sur 37 lus conformément au dessin.**

**Recouvrements** (liens posés dont le successeur commence au plus tard le jour de fin du prédécesseur, jalons compris) : R1 10 sur 109, R3 18 sur 177, R5 7 sur 47 ; décalages négatifs effectivement posés (R-280) : 1, 9 et 1 (les autres : jalons du même jour, ou tenus par une contrainte). Recouvrement voulu ou erreur de relevé : **inconnu** (aucun fichier source) ; les tâches concernées se chevauchent dans le dessin.

### 11.4 Effet sur les dates identiques au PDF

`cargo test --release -p plannercraft-pdfimport --test corpus pdf_corpus_links_dates -- --nocapture` ; PDF à colonne : colonne retirée pour mesurer le dessin seul. Dates calculées égales aux dates lues au jour, sans et avec les liens dessinés (R-278 à R-280).

| Planning | Fériés sans liens | Fériés avec liens | Fériés + périodes sans liens | Fériés + périodes avec liens |
|---|---|---|---|---|
| R1 | 308/335 | 308/335 | 322/335 | 322/335 |
| R3 | 405/438 | 405/438 | 425/438 | 425/438 |
| R5 | 106/124 | 106/124 | 112/124 | 112/124 |
| R2, R4, R6 | sans liens dessinés | inchangé | | |
| **Total R1 à R6** | **1 365/1 551 (88,0 %)** | **1 365 (88,0 %)** | **1 441 (92,9 %)** | **1 441 (92,9 %)** |

Corpus public, réglage par défaut (fériés seulement pour un en-tête français) : aucune perte ; gains : Exit 140 → 148 dates sur 210, Geneva 199 → 200 ; sur les PDF à colonne, le dessin seul fait mieux que la colonne avec R-271 stricte (Wellesley SD 365 contre 69 sur 366, Upham 236 contre 27, Warrington 851 contre 748), parce que R-279 et R-280 tiennent les débuts lus. Fériés français forcés sur un planning anglais (réglage non proposé) : Exit perd 1 date. Objectif (b), décaler un prédécesseur pousse ses successeurs une fois la marge consommée : test `r279_r280_delaying_the_predecessor_pushes_the_successor_once_slack_is_used`.

### 11.5 Temps et sauts de page

Temps (release, lecture complète ; entre parenthèses, relevé des liens coupé) : 1 à 25 ms de plus par fichier ; plus gros fichier importé du corpus (ICANN, 811 Ko, 6 pages, 132 pointes) 100 ms (96) ; plus coûteux, Salem 133 ms (108). Conversion avec liens : 2 à 33 ms, sauf R3 97 ms (un lien refusé : lot repris lien par lien).

Sauts de page (observé) : oui, un lien peut traverser un saut de page : sa descente touche le bas des lignes d'une page et reprend en haut de la suivante (Wellesley SD pages 3 et 4, Coquille, ICANN, Warrington, R1, R3). Non recousus : départ vertical sans barre, écarté et compté (`cut`).
