# Traductions anglaises de l'interface (repli sur fr.ftl pour une clé absente).
# Chaque clé correspond à une variante de `TextId` (src/i18n.rs) ; un test
# vérifie que fr.ftl et en.ftl déclarent exactement les mêmes clés.
# Pas de U+202F (absente des polices d'egui) ; points de suspension U+2026
# « … » (présents dans les polices d'egui, qui s'en servent pour tronquer).

## Écran d'accueil

welcome-title = Welcome to { $product }
welcome-subtitle = Free MS Project clone — open, edit, save
welcome-open-demo = Open the sample: house construction
welcome-open-file = Open an MSPDI file (.xml)…
welcome-new-project = New project
welcome-web-note = The file stays in your browser; nothing is uploaded.
welcome-language = Language
welcome-theme = Theme

## Menus

menu-file = File
menu-open = Open an MSPDI file (.xml)…
menu-open-demo = Open the sample
menu-close = Close
menu-view = View
menu-zoom = Zoom
menu-theme = Theme
menu-language = Language
menu-diagnostics = Diagnostics

## Zoom

toolbar-fit = Fit
zoom-day = Day
zoom-week = Week
zoom-month = Month
zoom-quarter = Quarter

## Thème et langue (noms de langue dans leur propre langue)

theme-system = System
theme-light = Light
theme-dark = Dark
lang-system = System
lang-fr = Français
lang-en = English

## Colonnes de la table

col-id = ID
col-name = Task Name
col-duration = Duration
col-start = Start
col-finish = Finish
col-predecessors = Predecessors
col-total-slack = Total Slack
col-critical = Critical
col-percent-complete = % Complete
col-resource-names = Resource Names
col-baseline-start = Baseline Start
col-baseline-finish = Baseline Finish
col-finish-variance = Finish Variance
yes = Yes
no = No

## Unités de durée (forme longue, hypothèse H-UI-04 : « 5 days », « 1 day? » ;
## écoulé avec le préfixe « e » de MS Project : « 5 edays », ADR-0011 décision 9)

unit-minute-one = minute
unit-minute-other = minutes
unit-hour-one = hour
unit-hour-other = hours
unit-day-one = day
unit-day-other = days
unit-week-one = week
unit-week-other = weeks
unit-month-one = month
unit-month-other = months
unit-minute-elapsed-one = eminute
unit-minute-elapsed-other = eminutes
unit-hour-elapsed-one = ehour
unit-hour-elapsed-other = ehours
unit-day-elapsed-one = eday
unit-day-elapsed-other = edays
unit-week-elapsed-one = eweek
unit-week-elapsed-other = eweeks
unit-month-elapsed-one = emonth
unit-month-elapsed-other = emonths

## Types de liens

link-fs = FS
link-ss = SS
link-ff = FF
link-sf = SF

## Conventions de formatage (lues par Texts::format_labels) ; le point est
## écrit en littéral fluent pour ne pas commencer une valeur par « . ».

fmt-decimal-sep = {"."}
fmt-list-sep = ,
fmt-date-order = mdy
fmt-unknown = ?
# Règle de pluriel des unités : below-two (singulier si |v| < 2) ou
# exactly-one (singulier si |v| = 1).
fmt-plural-rule = exactly-one

## Mois

month-1-long = January
month-1-short = Jan
month-2-long = February
month-2-short = Feb
month-3-long = March
month-3-short = Mar
month-4-long = April
month-4-short = Apr
month-5-long = May
month-5-short = May
month-6-long = June
month-6-short = Jun
month-7-long = July
month-7-short = Jul
month-8-long = August
month-8-short = Aug
month-9-long = September
month-9-short = Sep
month-10-long = October
month-10-short = Oct
month-11-long = November
month-11-short = Nov
month-12-long = December
month-12-short = Dec

## Jours de la semaine

weekday-mon-short = Mon
weekday-mon-initial = M
weekday-tue-short = Tue
weekday-tue-initial = T
weekday-wed-short = Wed
weekday-wed-initial = W
weekday-thu-short = Thu
weekday-thu-initial = T
weekday-fri-short = Fri
weekday-fri-initial = F
weekday-sat-short = Sat
weekday-sat-initial = S
weekday-sun-short = Sun
weekday-sun-initial = S

## Échelle de temps

tier-quarter = Q
tier-week = W

## Barre d'état

status-tasks =
    { $count ->
        [one] { $count } task
       *[other] { $count } tasks
    }
status-range = { $start } – { $finish }
status-errors =
    { $count ->
        [0] no errors
        [one] { $count } error
       *[other] { $count } errors
    }
status-warnings =
    { $count ->
        [0] no warnings
        [one] { $count } warning
       *[other] { $count } warnings
    }
status-consistent = Schedule consistent
status-no-document = No document open

## Ouverture de fichiers

open-error-title = Could not open the file
open-error-read = Could not read the file: { $detail }
open-error-mspdi = Unreadable MSPDI file: { $detail }

## Nouveau, Enregistrer, Enregistrer sous (lot L13)

menu-new = New
menu-save = Save
menu-quit = Quit
menu-save-as = Save As…
new-project-name = Project1
dialog-save-title = Save the project (MSPDI)
save-in-progress = A save is already in progress.
save-error-title = Could not save the file
save-error-write = Cannot write the file: { $detail }
save-adjusted =
    { $count ->
        [one] Saved: { $count } value adjusted to the MSPDI format.
       *[other] Saved: { $count } values adjusted to the MSPDI format.
    }
confirm-unsaved-title = Save changes to “{ $name }”?
confirm-save = Save
confirm-discard = Don't Save
confirm-cancel = Cancel

## Bandeau de messages

message-dismiss = Dismiss
range-truncated = Display range limited to 20 years

## Accessibilité et divers

row-label = Row { $id }: { $name }
splitter-label = Splitter between the table and the Gantt chart
gantt-scroll-label = Gantt chart horizontal scroll
demo-file-name = Sample: house construction
file-filter-mspdi = MSPDI file (XML)
dialog-open-title = Open an MSPDI file
state-load-warning = Display preferences ignored: { $detail }

## Menu Édition

menu-edit = Edit
menu-undo = Undo
menu-redo = Redo

## Structure (menu contextuel, confirmation de suppression)

structure-insert = Insert Task
structure-delete = Delete Task
structure-indent = Indent
structure-outdent = Outdent
structure-link = Link Tasks
structure-unlink = Unlink Tasks
confirm-delete-title = Delete a summary task
confirm-delete-text =
    { $count ->
        [one] The task will be deleted.
       *[other] The summary task and its subtasks will be deleted ({ $count } tasks).
    }
confirm-delete-yes = Delete
confirm-delete-no = Cancel

## Refus de commande (une clé par code d'erreur du moteur, `error-<code>`)

command-error-title = Change refused
error-unknown = Error: { $detail }
error-assignment-duplicate = Resource already assigned: { $detail }
error-assignment-kind_mismatch = Field does not match the resource type: { $detail }
error-assignment-not_found = Assignment not found: { $detail }
error-assignment-zero_units_fixed_work = Zero units on a fixed-work task: { $detail }
error-baseline-exists = Baseline already set: { $detail }
error-baseline-rollup_needs_selection = Roll-up needs a selection: { $detail }
error-batch-duplicate_label = Duplicate label: { $detail }
error-batch-unknown_label = Unknown label: { $detail }
error-batch-unsupported_version = Unsupported format version: { $detail }
error-batch-would_create_errors = Change would introduce errors: { $detail }
error-calendar-exception_not_found = Calendar exception not found: { $detail }
error-calendar-exception_overlap = Overlapping calendar exceptions: { $detail }
error-calendar-not_found = Calendar not found: { $detail }
error-command-not_yet_available = Command not available yet: { $detail }
error-explain-not_scheduled = Task not scheduled: { $detail }
error-file-invalid = Unreadable file: { $detail }
error-file-open_in_app = File open in the application: { $detail }
error-file-outside_root = Path not allowed: { $detail }
error-file-read = Read failed: { $detail }
error-file-too_large = File too large: { $detail }
error-file-write = Write failed: { $detail }
error-history-empty = Nothing to undo or redo: { $detail }
error-history-inconsistent = Inconsistent history: { $detail }
error-limit-exceeded = Limit exceeded: { $detail }
error-link-duplicate = Link already exists: { $detail }
error-link-not_found = Link not found: { $detail }
error-link-self = Task linked to itself: { $detail }
error-link-summary_descendant = Link between a summary task and its subtask: { $detail }
error-param-conflict = Mutually exclusive values: { $detail }
error-param-invalid = Value refused: { $detail }
error-param-mismatch = Contradictory values: { $detail }
error-progress-duration_below_actual = Duration below the actual duration: { $detail }
error-progress-inactive = Inactive task: { $detail }
error-progress-invalid_order = Actual finish before actual start: { $detail }
error-progress-summary_not_editable = Progress of a summary task: { $detail }
error-resource-has_assignments = Resource has assignments: { $detail }
error-resource-not_found = Resource not found: { $detail }
error-session-none = No document: { $detail }
error-session-poisoned = Session unavailable: { $detail }
error-task-computed_field = Computed field: { $detail }
error-task-fixed_work_effort_driven = Fixed work is always effort driven: { $detail }
error-task-has_children = Summary task with subtasks: { $detail }
error-task-manual_unsupported = Manually scheduled task: { $detail }
error-task-move_into_subtree = Move not possible: { $detail }
error-task-not_found = Task not found: { $detail }
error-task-outline_refused = Outline change refused: { $detail }
error-text-bad_date = Unreadable date: { $detail }
error-text-bad_link_type = Unknown link type: { $detail }
error-text-bad_number = Unreadable number: { $detail }
error-text-empty = Empty entry: { $detail }
error-text-external_predecessor = External predecessor: { $detail }
error-text-negative = Negative value: { $detail }
error-text-out_of_range = Value out of range: { $detail }
error-text-too_long = Entry too long: { $detail }
error-text-unknown_unit = Unknown unit: { $detail }

## Cell editing (lot L12)

blank-row-label = Blank row (new task)
cell-edit-label = Edit: { $column }
edit-error-unknown-id = No task with ID { $id }
edit-error-not-editable = Computed column, read-only

## J7 — L0: Contract refusal codes (ADR-0015)

error-calendar-name_taken = Calendar name already taken: { $detail }
error-calendar-base_not_base = Invalid base calendar: { $detail }
error-calendar-in_use = Calendar in use: { $detail }
error-gesture-not_draggable = Bar cannot be dragged this way: { $detail }
error-usage-limit = Usage report too large: { $detail }
error-control-rejected = Rejected in the application: { $detail }
error-control-approval_timeout = No answer in time: { $detail }
error-control-not_relayable = Command cannot be relayed: { $detail }
error-control-not_attached = No attached application: { $detail }
error-control-app_gone = Application closed: { $detail }
error-control-screenshot_unavailable = Screenshot unavailable: { $detail }
error-control-screenshot_too_large = Screenshot too large: { $detail }
error-control-busy = Application busy: { $detail }
error-control-document_changed = Application document changed: { $detail }

# --- J7 L1 ---
table-label = Task table
nav-cursor-label = Row { $id }, { $column }, { $value }
nav-cursor-blank-label = Blank row, { $column }
nav-cursor-empty-value = empty

# --- J7 L2 ---

# --- J7 L3 ---
approval-title = The agent proposes on “{ $document }”: { $name }
approval-changed =
    { $touched ->
        [0] no task changed
        [one] { $touched } task changed
       *[other] { $touched } tasks changed
    } · { $moved ->
        [0] no date moved
        [one] { $moved } date moved
       *[other] { $moved } dates moved
    }
approval-finish = finish { $value }
approval-finish-same = finish unchanged
approval-critical = critical path changed
approval-warnings =
    { $count ->
        [one] { $count } warning
       *[other] { $count } warnings
    }
approval-queued =
    { $count ->
        [one] { $count } more proposal waiting
       *[other] { $count } more proposals waiting
    }
approval-accept = Accept
approval-reject = Reject
menu-undo-agent = Undo: agent — { $detail }
menu-undo-agent-ops =
    { $count ->
        [one] { $count } operation
       *[other] { $count } operations
    }

# --- J7 L4 ---

# --- J7 L5 ---

# --- J7 L6 ---
fmt-percent-suffix = %
column-header-label = Column { $column }
column-resize-label = Width of the { $column } column
column-menu-hide = Hide column
table-scroll-label = Task table horizontal scroll

# --- J7 L7 ---
menu-gantt-standard = Gantt Chart
menu-gantt-tracking = Tracking Gantt

# --- J7 L8 ---
gantt-reason-summary = summary task (only linking is allowed)
gantt-reason-started = task already started
gantt-reason-milestone = milestone
gantt-reason-excluded = task excluded from scheduling
gantt-reason-blank = blank row
gantt-reason-overflow = offset outside the allowed dates
gantt-reason-same-task = task linked to itself
link-dialog-title = Task Dependency
link-dialog-from = From
link-dialog-to = To
link-dialog-type = Type
link-dialog-lag = Lag
link-dialog-delete = Delete
link-dialog-stale = The project changed: the dependency was reloaded.
link-dialog-task = { $id } { $name }

# --- J7 L9 ---
task-info-menu = Task Information…
task-info-title = Task Information — { $id } { $name }
task-info-tab-general = General
task-info-tab-predecessors = Predecessors
task-info-tab-resources = Resources
task-info-tab-advanced = Advanced
task-info-tab-notes = Notes
task-info-name = Name
task-info-duration = Duration
task-info-percent = % complete
task-info-start = Start
task-info-finish = Finish
task-info-active = Active
task-info-pred-id = ID
task-info-pred-type = Type
task-info-pred-lag = Lag
task-info-row-field = { $column }, row { $count }
task-info-add-predecessor = Add predecessor
task-info-add-resource = Assign resource
task-info-remove-row = Remove
task-info-resource-name = Resource
task-info-resource-units = Units
task-info-no-resource = No resources in the project.
task-info-constraint = Constraint type
task-info-constraint-date = Constraint date
task-info-deadline = Deadline
task-info-task-type = Task type
task-info-effort-driven = Effort driven
task-info-notes = Task notes
task-info-link-fs = Finish-to-Start (FS)
task-info-link-ss = Start-to-Start (SS)
task-info-link-ff = Finish-to-Finish (FF)
task-info-link-sf = Start-to-Finish (SF)
task-info-constraint-asap = As Soon As Possible
task-info-constraint-alap = As Late As Possible
task-info-constraint-mso = Must Start On
task-info-constraint-mfo = Must Finish On
task-info-constraint-snet = Start No Earlier Than
task-info-constraint-snlt = Start No Later Than
task-info-constraint-fnet = Finish No Earlier Than
task-info-constraint-fnlt = Finish No Later Than
task-info-type-fixed-units = Fixed Units
task-info-type-fixed-duration = Fixed Duration
task-info-type-fixed-work = Fixed Work
task-info-ok = OK
task-info-cancel = Cancel
task-info-stale = The project changed since this window opened: values were reloaded, nothing was applied.
task-info-date-required = A date is required for this constraint type
task-info-bad-id = Unreadable ID: { $id }
task-info-duplicate-predecessor = Duplicate predecessor: { $id }
task-info-duplicate-resource = Resource assigned twice: { $name }
task-info-unknown-resource = Unknown resource

# --- J7 L10 ---
menu-view-resource-sheet = Resource Sheet
res-sheet-label = Resource sheet
res-col-indicator = Ind.
res-col-name = Resource Name
res-col-kind = Type
res-col-material-label = Material Label
res-col-initials = Initials
res-col-group = Group
res-col-max-units = Max. Units
res-col-standard-rate = Std. Rate
res-col-overtime-rate = Ovt. Rate
res-col-cost-per-use = Cost/Use
res-col-calendar = Calendar
res-kind-work = Work
res-kind-material = Material
res-kind-cost = Cost
rate-minute = /min
rate-hour = /hr
rate-day = /day
rate-week = /wk
rate-month = /mon
rate-year = /yr
res-row-label = Resource { $id }: { $name }
res-blank-row-label = Blank row (new resource)
res-name-first = Enter the resource name first
res-unknown-calendar = No calendar named "{ $name }"
res-calendar-none = (none)
res-delete-title = Delete an assigned resource
res-delete-text =
    { $count ->
        [one] "{ $name }" is assigned to one task: the assignment will be removed with it.
       *[other] "{ $name }" has { $count } assignments: they will be removed with it.
    }

# --- J7 L11 ---
menu-view-resource-usage = Resource Usage
usage-sheet-label = Resource usage
usage-periods-label = Work per period
usage-col-expand = +/-
usage-col-name = Resource Name
usage-col-work = Work
usage-hours-suffix = h
usage-expand = Expand { $name }
usage-collapse = Collapse { $name }
usage-assignment-label = Assignment: { $name } ({ $detail })
usage-limit = Usage too large to display: collapse some resources ({ $detail }).

# --- J7 L12 ---
menu-project = Project
menu-project-info = Project Information…
menu-working-time = Change Working Time…
menu-baseline = Baseline…
menu-reschedule = Reschedule Uncompleted Work…
scope-project = Entire project
scope-selection = { $count ->
        [one] Selected task (1)
       *[other] Selected tasks ({ $count })
    }
wt-title = Change Working Time
wt-calendar = For calendar
wt-project-calendar = { $name } (Project Calendar)
wt-resource-calendar = { $name } (resource)
wt-new = Create New Calendar…
wt-delete = Delete calendar
wt-name = Calendar name
wt-base-of = Base calendar: { $name }
wt-new-name = New calendar name
wt-new-base = Create new base calendar
wt-new-copy = Make a copy of “{ $name }”
wt-create = Create
wt-new-cancel = Do not create
wt-prev-month = Previous month
wt-next-month = Next month
wt-month-title = { $name } { $value }
wt-month-unavailable = Calendar unavailable: { $detail }
wt-state-working = Working: { $value }
wt-state-non-working = Nonworking
wt-legend-working = Working
wt-legend-non-working = Nonworking
wt-legend-exception = Exception
wt-tab-exceptions = Exceptions
wt-tab-week = Work Week
wt-exc-name = Name
wt-exc-from = Start
wt-exc-to = Finish
wt-exc-working = Working
wt-exc-ranges = Times
wt-exc-recurring = Recurring (read only)
wt-exc-add = Add exception
wt-week-inherit = Inherited from base
wt-week-non-working = Nonworking day
wt-week-working = Specific working times
wt-week-setting = Setting for { $name }
wt-week-ranges = Times for { $name }
wt-week-inherited = Base: { $value }
wt-bad-ranges = Times expected as 08:00-12:00, 13:00-17:00
pi-title = Project Information
pi-direction = Schedule from
pi-from-start = Project Start Date
pi-from-finish = Project Finish Date
pi-start = Start date
pi-finish = Finish date
pi-status = Status date
pi-status-hint = Empty: NA (today serves as status date)
pi-calendar = Calendar
bl-title = Baseline
bl-set = Set baseline
bl-clear = Clear baseline
bl-number = Number
bl-baseline-0 = Baseline
bl-baseline-n = Baseline { $count }
bl-rollup = Roll up baselines
bl-rollup-none = No rollup
bl-rollup-all = To all summary tasks
bl-rollup-from = From subtasks into selected summary tasks
bl-confirm = { $detail } Replace the saved baseline?
bl-replace = Replace
rs-title = Reschedule Uncompleted Work
rs-after = Reschedule after

# --- J7 L13 ---
menu-assign-resources = Assign Resources…
as-title = Assign Resources
as-task-one = Task: { $name }
as-task-many = { $count ->
        [one] { $count } selected task
       *[other] { $count } selected tasks
    }
as-no-task = No task selected
as-filter = Filter by name
as-resource = Resource Name
as-units = Units
as-units-of = Units of { $name }
as-assigned = Assigned
as-assigned-all = yes
as-assigned-some = { $count } of { $value }
as-assign = Assign
as-remove = Remove
as-update-units = Change Units
as-close = Close
as-no-resource = No resource
as-hint = Click a resource to choose it (Ctrl or Cmd + click: several); units left empty: default value.

# --- J7 L14 ---
open-error-read-file = Could not read “{ $name }”: { $detail }
open-error-mspdi-file = “{ $name }”: unreadable MSPDI file: { $detail }
open-error-missing = “{ $name }” was not found and was removed from recent files: { $detail }
menu-recent = Recent files
menu-recent-empty = No recent files
menu-recent-missing = { $name } (not found)

# --- J7 L15 ---
recovery-title = Recover “{ $name }” ({ $count ->
        [one] { $count } change
       *[other] { $count } changes
    })?
recovery-checkpointed = Changes made before the last recovery point can no longer be undone.
recovery-restore = Restore
recovery-discard = Discard
recovery-later = Later
recovery-partial = “{ $name }” partly recovered: { $count ->
        [one] { $count } change replayed
       *[other] { $count } changes replayed
    }, the next one was refused: { $detail }
recovery-failed = Cannot recover “{ $name }”: { $detail }
recovery-discard-failed = Cannot delete the recovery journal: { $detail }
recovery-journal-cut = Crash recovery stopped for this document: { $detail }. Editing continues; save often.
recovery-file-changed = The file changed on disk since then: the recovered version opens without a path (Save As), so it cannot overwrite it.
recovery-kept = Journal from another PlannerCraft version, or too large: it cannot be restored here. Discard deletes it.
recovered-file-name = { $stem } (recovered)

# --- J7 L16 ---

# --- J8 L0 : codes de refus du contrat (ADR-0016) ---
error-paste-unrecognized = Pasted text not recognized: { $detail }
error-paste-too_large = Pasted text too large: { $detail }
error-paste-links_dropped = Links not copied: { $detail }
error-style-ignored = File formatting ignored: { $detail }
error-style-invalid = Invalid formatting: { $detail }
error-pdf-too_many_pages = PDF too long: { $detail }
error-pdf-glyph_missing = Character missing from the PDF fonts: { $detail }
error-pdf-logo_invalid = Unreadable logo: { $detail }
error-task-manual_finish_before_start = Manual task finish before its start: { $detail }
error-control-bad_endpoint = Invalid control channel address: { $detail }
error-custom-unknown_field = Unknown custom field: { $detail }
error-custom-value_type = Custom field value refused: { $detail }
error-custom-too_long = Text too long: { $detail }
error-custom-read_only = Field defined by the file: { $detail }

# --- J8 L1 ---

# --- J8 L2 ---

# --- J8 L3 ---

# --- J8 L4 ---

# --- J8 L5 ---

# --- J8 L6 ---

# --- J8 L7 ---

# --- J8 L8 ---

# --- J8 L9 ---
menu-filter = Filter
filter-all = All tasks
filter-critical = Critical tasks
filter-late = Late tasks
filter-incomplete = Incomplete tasks
filter-milestones = Milestones
filter-summaries = Summary tasks
filter-using-resource = Using resource…
filter-date-range = Date range…
filter-custom = Custom filter…
filter-reapply = Reapply filter
filter-clear = Clear filter
filter-using-named = Using { $name }
status-filter = Filter: { $name }
filter-custom-title = Custom filter
filter-resource-title = Using resource
filter-date-range-title = Date range
filter-apply = Apply
filter-resource = Resource
filter-no-resource = No resource in the project
filter-use-resource = Assigned to the resource
filter-from = From
filter-to = To
filter-active-only = Active tasks
filter-outline-max = Outline level at most
filter-name-contains = Name containing
filter-related = Show related summary rows
filter-date-order = The first day is after the last one.
filter-dates-required = Enter the first and the last day.
filter-resource-missing = Choose a resource.
menu-sort = Sort…
menu-sort-by-id = Sort by ID
sort-title = Sort
sort-by = Sort by
sort-ascending = Ascending
sort-descending = Descending
sort-keep-outline = Keep outline structure
sort-apply = Sort tasks
sort-reset = Reset (ID order)
menu-outline-level = Outline level
outline-level-n = Level { $count }
outline-all = All levels
menu-hide-subtasks = Hide subtasks
menu-show-subtasks = Show subtasks
outline-expand-row = Expand { $name }
outline-collapse-row = Collapse { $name }
view-reorder-locked = Remove the sort or the filter to move, indent or outdent tasks.
menu-find = Find…
search-label = Find
search-previous = Previous
search-next = Next
search-all-columns = Visible columns
search-close = Close search
search-none = No match
search-counter = { $id }/{ $count }
search-hidden =
    { $count ->
        [one] ({ $count } hidden by the filter)
       *[other] ({ $count } hidden by the filter)
    }

# --- J8 L10 ---
menu-cut = Cut
menu-copy = Copy
menu-paste = Paste
menu-paste-special = Paste Special…
menu-import-csv = Import CSV…
paste-report-title = Paste
paste-report-tasks = { $count ->
        [one] { $count } task pasted
       *[other] { $count } tasks pasted
    }
paste-report-links = { $count ->
        [one] { $count } link to a task that was not copied not kept
       *[other] { $count } links to tasks that were not copied not kept
    }
paste-report-assignments = { $count ->
        [one] { $count } assignment skipped (missing resource)
       *[other] { $count } assignments skipped (missing resources)
    }
paste-report-resources = { $count ->
        [one] { $count } resource created
       *[other] { $count } resources created
    }
paste-hint-unrecognized = paste a header row (Name, Duration…) followed by the rows, or a single column of names
paste-hint-too-large = 10,000 lines and 4 MiB at most
paste-use-shortcut = The browser does not give the clipboard to a menu: paste with Ctrl+V (Cmd+V on Mac).
ps-title = Paste Special
ps-source = { $count ->
        [one] Last copy: { $count } task
       *[other] Last copy: { $count } tasks
    }
ps-links = Recreate the links between the pasted tasks
ps-resources = Assignments
ps-res-match = By resource name (skipped if the resource does not exist)
ps-res-create = By name, creating missing resources
ps-res-drop = Do not paste assignments
ps-nothing = Nothing to paste: copy rows first (Ctrl+C).
ci-pick-title = Import a CSV file
ci-pick-filter = CSV files
ci-title = Import CSV
ci-file = File: { $name }
ci-separator = Separator
ci-encoding = Encoding
ci-language = Language
ci-auto = Automatic
ci-sep-semicolon = Semicolon (;)
ci-sep-comma = Comma (,)
ci-sep-tab = Tab
ci-enc-utf8 = UTF-8
ci-enc-utf16le = UTF-16 LE
ci-enc-utf16be = UTF-16 BE
ci-enc-windows1252 = Windows-1252
ci-detected = Using: { $detail }
ci-columns = Columns
ci-rows = { $count ->
        [one] Preview ({ $count } row)
       *[other] Preview ({ $count } rows)
    }
ci-recognized = recognized
ci-unknown = ignored (unknown)
ci-unsupported = not supported
ci-duplicate = ignored (duplicate)
ci-diagnostics = { $count ->
        [one] { $count } remark on the rows shown
       *[other] { $count } remarks on the rows shown
    }
ci-diagnostic-line = Line { $count }: { $detail }
ci-import = Import
ci-imported = { $count ->
        [one] “{ $name }” imported with { $count } remark (default value or ignored column).
       *[other] “{ $name }” imported with { $count } remarks (default values or ignored columns).
    }
error-csv-error-too_large = CSV file too large (64 MiB at most).
error-csv-error-empty = Empty CSV file.
error-csv-error-no_header = No header row in the file.
error-csv-error-no_known_column = No Name or Duration column in the header.
error-csv-error-too_many_rows = Too many rows (200,000 at most).
error-csv-error-too_many_columns = Too many columns (256 at most).
error-csv-error-encoding = The file is not in the selected encoding.
error-csv-error-not_yet_available = Feature not available yet.

# --- J8 L11 ---
col-mode = Task Mode
fmt-mode-manual = Manually Scheduled
fmt-mode-auto = Auto Scheduled
gantt-status-label = Status { $date }

# --- J8 L12 ---
menu-ai-panel = AI operations
side-tab-ai = AI operations
side-tab-why = Why this date?
side-collapse = Collapse the pane
ai-no-pending = No proposal pending.
ai-more =
    { $count ->
        [one] { $count } more changed task
       *[other] { $count } more changed tasks
    }
ai-critical-entered = becomes critical
ai-critical-left = no longer critical
ai-added = + { $name }
ai-removed = deleted: { $name }
ai-no-preview = No preview for this operation: { $detail }
ai-history = Session history
ai-history-empty = No proposal handled yet.
ai-history-line = { $value } — { $name } on "{ $document }": { $detail }
ai-outcome-accepted = accepted
ai-outcome-rejected = rejected
ai-outcome-expired = expired
ai-outcome-stale = stale (document closed or replaced)
ai-outcome-failed = failed ({ $detail })
ai-undo-entry = Undo this operation
why-no-task = Select a task to see what sets its dates.
why-drivers = Drivers (click to select)
why-unavailable = No explanation: { $detail }

# --- J8 L13 ---
web-start-failed = { $product } cannot start in this browser (WebGL2 or WebGPU unavailable?). Try a recent browser or the desktop version. Detail: { $detail }
dropped-first-only = { $count } files dropped: only "{ $name }" is opened.
menu-export = Export
menu-export-pdf = PDF…
menu-export-svg = SVG…
menu-export-csv = Tasks CSV…
export-done = Exported: { $name }
export-error-title = Cannot export
export-error-failed = The export failed: { $detail }

# --- J8 L14 ---
diag-calendar-unknown = unknown calendar { $calendar }, referenced by { $referenced_by }
diag-calendar-base_missing = calendar { $calendar } inherits from calendar { $base }, which does not exist
diag-calendar-base_not_base = calendar { $calendar } inherits from calendar { $base }, which is not a base calendar
diag-calendar-inheritance_cycle = inheritance loop between calendars: { $chain }
diag-calendar-no_working_time = calendar { $calendar } has no working time: no duration can be placed on it
diag-calendar-internal = calendar { $calendar } could not be compiled (internal inconsistency, please report)
diag-calendar-exceptions_overlap = calendar { $calendar }: exceptions #{ $kept } and #{ $ignored } overlap; #{ $kept } wins on the shared days
diag-calendar-work_weeks_overlap = calendar { $calendar }: work weeks #{ $kept } and #{ $ignored } overlap; #{ $kept } wins on the shared days
diag-calendar-recurrence_unsupported = calendar { $calendar }: the recurrence of exception #{ $exception } is not supported; the exception is ignored
diag-project-calendar_not_base = the project calendar ({ $calendar }) is not a base calendar
diag-task-reserved_uid = a task has UID 0, reserved for the project summary task
diag-task-outline_jump = task { $task } skips an outline level (or the first level is not 1)
diag-task-null_with_data = blank row { $task } carries links or assignments, ignored
diag-task-negative_progress = task { $task } has a negative actual or remaining duration
diag-link-self = task { $task } is linked to itself
diag-link-duplicate = several links from task { $predecessor } to task { $successor }
diag-link-unknown_task = task { $successor } is linked to task { $predecessor }, which does not exist
diag-assignment-unknown_task = assignment { $assignment } refers to task { $task }, which does not exist
diag-assignment-unknown_resource = assignment { $assignment } refers to resource { $resource }, which does not exist
diag-conversion-out_of_domain = calendar { $calendar }: { $minutes } min from { $from } fall outside the date range (1984-2149)
diag-link-cycle = circular links between tasks { $tasks }
diag-link-ignored_in_cycle = the link { $predecessor } -> { $successor } is ignored to break a cycle
diag-link-summary_descendant = the link { $predecessor } -> { $successor } joins a summary task to one of its subtasks: ignored
diag-link-finish_to_summary = the link { $predecessor } -> { $successor } (finish-to-finish or start-to-finish) to a summary task applies to each of its subtasks
diag-link-expansion_limit = { $ignored ->
        [one] { $ignored } finish-to-finish or start-to-finish link to a summary task ignored
       *[other] { $ignored } finish-to-finish or start-to-finish links to a summary task ignored
    }: expanding them to the subtasks would exceed { $limit } arcs
diag-link-start_from_summary = the link { $predecessor } -> { $successor } (start-to-start or start-to-finish) from a summary task caps the start of each of its subtasks (scheduled from finish)
diag-link-inactive = the link { $predecessor } -> { $successor } starts from an inactive task: no effect
diag-link-excluded_endpoint = the link { $predecessor } -> { $successor } touches an unscheduled task: ignored
diag-link-manual_not_honored = manual task { $successor } does not honor its link with task { $predecessor }
diag-link-unschedulable = the link { $predecessor } -> { $successor } to a summary task cannot be applied ({ $cause }): ignored
diag-task-manual_undated = manual task { $task } has neither start nor finish: not scheduled
diag-task-manual_dates_inverted = manual task { $task } finishes before it starts: finish moved to the start
diag-task-calendar_fallback = calendar { $calendar } of task { $task } is unusable: project calendar used
diag-project-calendar_fallback = the project calendar ({ $calendar }) is unusable: built-in Standard calendar used
diag-task-unschedulable = task { $task } cannot be scheduled ({ $pass }: { $cause })
diag-constraint-override = the "{ $kind }" constraint of task { $task } is honored despite { $bound }
diag-constraint-not_honored = the "{ $kind }" constraint on { $date } of task { $task } is not honored: links take precedence
diag-constraint-summary_ignored = the "{ $kind }" constraint of summary task { $task } is ignored
diag-constraint-manual_ignored = the "{ $kind }" constraint of manual task { $task } is ignored by scheduling
diag-task-deadline_missed = task { $task } finishes on { $finish }, after its deadline of { $deadline }
diag-resource-overallocated = resource { $resource } is overallocated from { $from } to { $to } (peak { $peak }%)
diag-resource-availability_ignored = dated availability of resource { $resource } is ignored: constant capacity
diag-assignment-contour_unsupported = contour { $contour } of assignment { $assignment } is placed as flat
diag-assignment-zero_units = assignment { $assignment } has zero units: the task duration is kept
diag-assignment-duration_out_of_domain = the duration of assignment { $assignment } is out of range
diag-assignment-calendar_empty = calendars { $task_calendar } (task) and { $resource_calendar } (resource) of assignment { $assignment } share no working time
diag-task-fixed_work_effort_driven = task { $task } is fixed work without being effort driven
diag-calendar-too_many_segments = the combination of calendars { $calendar } and { $other } exceeds { $limit } segments
diag-assignment-too_many_parts = task { $task } has { $parts } work assignments (more than { $limit }): its duration is that of the longest
diag-progress-link_ignored = the link { $predecessor } -> { $successor } has no effect: in-progress task { $successor } is not split
diag-progress-link_overridden = the actual dates of completed task { $successor } override its link with task { $predecessor }
diag-progress-option_unsupported = progress option { $option } is not followed: default values used
diag-progress-from_finish_in_progress = in-progress task { $task } in a project scheduled from finish: its remaining work is placed as soon as possible
diag-progress-actual_start_missing = started task { $task } has no actual start: its scheduled start is used
diag-progress-milestone_rounded = milestone { $task } is set to 100% complete
diag-constraint-ignored_in_progress = the "{ $kind }" constraint of in-progress task { $task } is ignored for its dates
diag-baseline-cost_not_computed = baseline { $number }: costs copied as is, not recalculated
diag-baseline-task_skipped = { $count ->
        [one] { $count } unscheduled task without a baseline
       *[other] { $count } unscheduled tasks without a baseline
    }
diag-baseline-nothing_to_clear = baseline { $number }: nothing to clear
diag-schedule-internal = internal scheduling inconsistency (please report): { $detail }
diag-task-custom_invalid = task { $task } has a { $field } value of the wrong type or out of range
diag-mspdi-invalid_value = unreadable or out-of-range value: default applied
diag-mspdi-unknown_code = unknown code ({ $code }): default applied
diag-mspdi-missing_required = required element missing
diag-mspdi-duplicate = duplicate element: the first one is kept
diag-mspdi-dangling_reference = reference to a missing UID
diag-mspdi-link_ignored = link ignored ({ $reason })
diag-mspdi-rounded = value rounded to the minute
diag-mspdi-unsupported = unsupported feature ({ $what })
diag-mspdi-inconsistent = tolerated inconsistency
diag-mspdi-truncated = { $dropped ->
        [one] { $dropped } diagnostic dropped beyond the limit
       *[other] { $dropped } diagnostics dropped beyond the limit
    }
diag-mspdi-not_preserved = content not read, lost on save
diag-mspdi-honor_constraints_off = "Honor constraints" turned off: links take precedence over constraints
diag-mspdi-progress_inconsistent = { $count ->
        [one] { $count } remaining value read from the file replaced by the calculation
       *[other] { $count } remaining values read from the file replaced by the calculation
    }
diag-mspdi-custom_unreadable = unreadable custom field value: kept as is in the file
diag-style-ignored = document style ignored ({ $reason }): default template applied
diag-mspdi-error-empty = empty document
diag-mspdi-error-too_large = document too large: { $size } bytes (limit { $max })
diag-mspdi-error-utf16_not_supported = UTF-16 document: only UTF-8 is supported
diag-mspdi-error-invalid_utf8 = invalid UTF-8 at { $at }
diag-mspdi-error-xml = malformed XML at { $at }: { $message }
diag-mspdi-error-unexpected_eof = unexpected end of document at { $at }{ $depth ->
        [0] {""}
       *[other] {" "}(open elements: { $open })
    }
diag-mspdi-error-doctype_not_allowed = DOCTYPE declaration refused at { $at }
diag-mspdi-error-too_deep = nesting too deep at { $at } (limit { $max } levels)
diag-mspdi-error-too_many_elements = too many elements (limit { $max })
diag-mspdi-error-too_many_retained = too many retained elements at { $at } (limit { $max })
diag-mspdi-error-text_too_long = text too long at { $at } (limit { $max } bytes)
diag-mspdi-error-not_mspdi = not an MSPDI document: root "{ $root }" instead of "Project"
diag-location = line { $line }, column { $column } (byte { $byte })
diag-subject-project = the project
diag-subject-calendar = calendar { $id }
diag-subject-task = task { $id }
diag-subject-link = link { $predecessor } -> { $successor }
diag-subject-resource = resource { $id }
diag-subject-assignment = assignment { $id }
diag-pass-forward = forward pass
diag-pass-backward = backward pass
diag-bound-project-start = the project start
diag-bound-project-finish = the project finish
diag-bound-summary-gate = the bound of summary task { $summary }
diag-bound-summary-cap = the cap of summary task { $summary }
diag-bound-predecessor = the link from task { $predecessor }
diag-bound-successor = the link to task { $successor }
diag-with-element = { $head }: { $element }
diag-with-value = { $head } = "{ $value }"
diag-with-detail = { $head }, { $detail }
template-classique = Classic
template-sobre = Clean
template-contraste-eleve = High contrast
template-presentation = Presentation
template-chantier = Works schedule
template-impression-nb = Black and white print
token-project = Project name
token-status-date = Status date
token-page = Page number
token-pages = Page count
token-print-date = Print date
token-file = File name

# --- J8 L15 ---
menu-task = Task
menu-format = Format
menu-help = Help
menu-mark-percent = Mark { $percent }% complete
menu-toggle-mode = Toggle manual / auto scheduling
menu-toggle-active = Activate / inactivate
menu-page-setup = Page Setup…
menu-print-mode = Print mode
menu-gallery = Layout templates…
menu-format-bars = Format bars…
menu-chart-only = Chart only
menu-highlight-critical = Highlight the critical path
menu-about = About { $product }
about-text = { $product } { $version } — free software (MIT or Apache-2.0)
context-format-bars = Format bars of this type
task-cmd-nothing = Nothing to change in the selection.
task-cmd-no-selection = Select one or more tasks first.
gallery-title = Layout templates
gallery-apply = Apply
gallery-current = Current template
gallery-default = Default
gallery-make-default = Set as default
gallery-default-hint = Template of documents without a chosen layout
gallery-user-templates = Personal templates
gallery-no-user-template = No personal template.
gallery-save-name = Template name
gallery-save = Save as personal template
gallery-saved = Personal template “{ $name }” saved.
gallery-name-invalid = A name of 1 to 64 characters is expected.
gallery-full = 32 personal templates at most: replace one (same name).
gallery-close = Close
gallery-preview-empty = No preview
page-setup-title = Page Setup
print-mode-title = Print mode
ps-paper = Paper
ps-landscape = Landscape
ps-portrait = Portrait
ps-margins = Margins (mm)
ps-margin-top = Top
ps-margin-right = Right
ps-margin-bottom = Bottom
ps-margin-left = Left
ps-fit = Fit
ps-fit-scale = Scale (%)
ps-fit-wide = Pages wide
ps-fit-wide-tall = Pages wide and tall
ps-pages-tall = Pages tall
ps-repeat-table = Table repeated on every page
ps-header = Header
ps-footer = Footer
ps-zone-left = Left
ps-zone-center = Center
ps-zone-right = Right
ps-insert-token = Insert into the header center
ps-title-block = Title block: first center line as a title
ps-columns = Printed columns
ps-chart-only = Chart only (table hidden)
ps-preview = Preview
ps-page-of = Page { $page } of { $pages }
ps-prev = Previous page
ps-next = Next page
ps-scale = Scale: { $scale }%
ps-export-pdf = Export to PDF…
ps-export-svg = Export to SVG…
ps-save = Save with the project
ps-close = Close
ps-error = Page layout failed: { $detail }
ps-column-name = Name
fp-title = Format: { $role }
fp-role = Bar type
fp-role-task = Task bars
fp-role-critical = Critical bars
fp-role-summary = Summary bars
fp-role-summary-critical = Critical summary bars
fp-role-milestone = Milestones
fp-role-milestone-critical = Critical milestones
fp-role-progress = Progress
fp-role-baseline = Baseline
fp-role-inactive = Inactive tasks
fp-role-manual = Manually scheduled tasks
fp-role-slack = Slack
fp-role-deadline = Deadline
fp-role-count =
    { $count ->
        [0] No bar of this type in this schedule.
        [one] { $count } bar of this type in this schedule.
       *[other] { $count } bars of this type in this schedule.
    }
fp-color = Color
fp-custom-color = Custom color
fp-swatch = Color { $color }
fp-phase-swatch = Phase color { $color }
fp-thickness = Thickness (% of the row)
fp-ends = Ends
fp-ends-square = Square
fp-ends-rounded = Rounded
fp-text-left = Text on the left
fp-text-right = Text on the right
fp-text-inside = Text inside
fp-none = None
fp-duration-after-name = Duration after the name
fp-details = Details
fp-contrast-low = Contrast too low { $bg ->
        [dark] on the dark background
        [print] in print
       *[light] on the light background
    }: { $ratio }:1 (3:1 at least).
fp-use-suggestion = Use { $color }
fp-revert = Revert to template
fp-save-template = Save as template…
fp-close = Close
fp-phase = Phase color of “{ $name }”
fp-phase-children = Applied to subtasks
fp-phase-remove = Remove the color
field-name = Name
field-name-duration = Name (duration)
field-resources = Resources
field-start = Start
field-finish = Finish
field-start-short = Start (short)
field-finish-short = Finish (short)
field-duration = Duration
field-duration-short = Duration (short)
field-percent = % complete
field-total-slack = Total slack
field-start-finish = Start – finish

# --- J8 L16 ---

# --- J8 relecture finale ---
paste-report-tracking = { $count ->
        [one] { $count } task pasted without its progress, baseline or calendar
       *[other] { $count } tasks pasted without their progress, baseline or calendar
    }
token-schedule-title = Title “Provisional schedule”
menu-relative-months = Relative months (M1, M2…)

# --- RUBAN L0 (ADR-0017) ---
ribbon-tab-task = Task
ribbon-tab-resource = Resource
ribbon-tab-project = Project
ribbon-tab-view = View
ribbon-tab-format = Format
ribbon-group-view = View
ribbon-group-clipboard = Clipboard
ribbon-group-formatting = Formatting
ribbon-group-schedule = Schedule
ribbon-group-tasks = Tasks
ribbon-group-insert = Insert
ribbon-group-properties = Properties
ribbon-group-editing = Editing
ribbon-group-assignments = Assignments
ribbon-group-status = Status
ribbon-group-print = Print
ribbon-group-task-views = Task Views
ribbon-group-resource-views = Resource Views
ribbon-group-data = Data
ribbon-group-zoom = Zoom
ribbon-group-split-view = Split View
ribbon-group-interface = Interface
ribbon-group-columns = Columns
ribbon-group-bar-styles = Bar Styles
ribbon-group-chart-style = Gantt Chart Style
ribbon-group-show-hide = Show/Hide
ribbon-gallery-short = Templates…
ribbon-percent = { $percent }%
ribbon-link = Link
ribbon-unlink = Unlink
ribbon-indent = Indent
ribbon-outdent = Outdent
ribbon-inactivate = Inactivate
ribbon-manual-schedule = Manually Schedule
ribbon-insert-task = Task
ribbon-insert-summary = Summary
ribbon-insert-milestone = Milestone
ribbon-information = Information
ribbon-notes = Notes
ribbon-find = Find
ribbon-scroll-to-task = Scroll to Task
ribbon-set-baseline = Set Baseline
ribbon-update-project = Update Project
ribbon-export-pdf = Export to PDF…
ribbon-sort = Sort
ribbon-outline = Outline
ribbon-filter = Filter
ribbon-timescale = Timescale
ribbon-zoom-in = Zoom In
ribbon-zoom-out = Zoom Out
ribbon-entire-project = Entire Project
ribbon-timeline = Timeline
ribbon-collapse = Collapse the Ribbon
ribbon-columns = Columns
ribbon-critical-tasks = Critical Tasks
ribbon-tip-save = Saves the project.
ribbon-tip-undo = Undoes the last change.
ribbon-tip-redo = Redoes the undone change.
ribbon-tip-gantt-chart = Shows the tasks and their bars over time.
ribbon-tip-tracking-gantt = Compares the scheduled bars with the baseline and shows progress.
ribbon-tip-resource-sheet = Lists the project resources.
ribbon-tip-resource-usage = Shows the work assigned to each resource, period by period.
ribbon-tip-format-bars = Changes the color, thickness and texts of the bars.
ribbon-tip-view-menu = Chooses the main view.
ribbon-tip-paste = Pastes the copied tasks before the selected row.
ribbon-tip-cut = Copies the selected tasks and deletes them.
ribbon-tip-copy = Copies the selected tasks.
ribbon-tip-paste-special = Chooses what the paste brings along.
ribbon-tip-gallery = Applies a formatting template to the schedule.
ribbon-tip-link = Links the selected tasks finish-to-start, in selection order.
ribbon-tip-unlink = Removes the links between the selected tasks.
ribbon-tip-indent = Makes the selected tasks subtasks of the task above.
ribbon-tip-outdent = Moves the selected tasks up one outline level.
ribbon-tip-inactivate = Keeps the task in the plan without counting it in the schedule.
ribbon-tip-manual-schedule = Switches the selected tasks between manual and automatic scheduling.
ribbon-tip-insert-task = Inserts a task before the selected row.
ribbon-tip-insert-summary = Inserts a summary task above the selected tasks.
ribbon-tip-insert-milestone = Inserts a zero-duration milestone before the selected row.
ribbon-tip-information = Opens the information of the active task.
ribbon-tip-notes = Opens the notes of the active task.
ribbon-tip-add-to-timeline = Adds the selected tasks to the timeline, or removes them. The timeline is saved with the project style.
ribbon-tip-find = Finds text in the tasks.
ribbon-tip-scroll-to-task = Scrolls the Gantt chart to the bar of the active task.
ribbon-tip-assign-resources = Assigns resources to the selected tasks.
ribbon-tip-project-information = Project dates, calendar and status date.
ribbon-tip-working-time = Working days and hours, holidays.
ribbon-tip-set-baseline = Saves the current schedule as a baseline.
ribbon-tip-update-project = Reschedules the remaining work after a date.
ribbon-tip-print-mode = Prepares printing: columns, page and preview.
ribbon-tip-page-setup = Paper, margins, header and footer.
ribbon-tip-export-pdf = Saves the schedule as a PDF.
ribbon-tip-sort = Sorts the tasks, or returns to ID order.
ribbon-tip-outline = Shows the tasks down to an outline level.
ribbon-tip-filter = Shows only the tasks that meet a criterion.
ribbon-tip-timescale = Unit of the timescale.
ribbon-tip-zoom-in = Shows smaller time units.
ribbon-tip-zoom-out = Shows larger time units.
ribbon-tip-entire-project = Fits the timescale to the whole project.
ribbon-tip-timeline = Shows the timeline above the Gantt chart.
ribbon-tip-ai-panel = Shows the AI agent operations pane.
ribbon-tip-diagnostics = Shows the project errors and warnings.
ribbon-tip-theme = Light, dark or system theme.
ribbon-tip-language = Interface language.
ribbon-tip-collapse = Shows only the tabs; clicking a tab opens the ribbon over the view.
ribbon-tip-columns = Shows or hides table columns.
ribbon-tip-critical-tasks = Highlights the tasks on the critical path.
ribbon-tip-relative-months = Numbers the months from the project start at the top of the timescale.
ribbon-tip-chart-only = Reduces the table to ID and name.
ribbon-reason-no-document = Open or create a project.
ribbon-reason-no-selection = Select at least one task.
ribbon-reason-nothing-to-undo = Nothing to undo.
ribbon-reason-nothing-to-redo = Nothing to redo.
ribbon-reason-task-view = Switch to a task view (Gantt Chart).
menu-insert-summary = Insert Summary Task
menu-insert-milestone = Insert Milestone
menu-task-notes = Notes…
timeline-add = Add to Timeline
timeline-remove = Remove from Timeline
timeline-too-many = The timeline holds at most { $max } tasks: nothing was added.
timeline-empty-hint = Right-click a task, then “Add to Timeline”

# --- RUBAN L1 ---

# --- RUBAN L2 ---

# --- RUBAN L3 ---
new-milestone-name = New milestone
new-summary-name = New summary task
insert-summary-not-adjacent = Select adjacent tasks at the same level to group them under a summary task.
scroll-task-hidden = The active task is hidden by the view (filter or outline).

# --- RUBAN L4 ---

# --- RUBAN L5 ---

# --- UX L0 (ADR-0018) ---
ux-not-available = This gesture is not available yet.
cells-too-large = The selection has more than { $max } cells: nothing was changed.
cells-parse = Paste refused: unreadable text for task { $id }, column “{ $column }” ({ $detail }). Nothing was changed.
cells-past-end = Cannot add tasks at the end of the project while a sort or filter is applied: nothing was changed.
cells-nothing-above = No row above to fill down from.
cells-skipped =
    { $count ->
        [one] { $count } read-only cell was skipped.
       *[other] { $count } read-only cells were skipped.
    }
cells-not-clearable =
    { $count ->
        [one] { $count } cell cannot be empty.
       *[other] { $count } cells cannot be empty.
    }
tasks-deleted =
    { $count ->
        [one] { $count } task deleted — { $undo } undoes.
       *[other] { $count } tasks deleted — { $undo } undoes.
    }
rows-move-inside-block = Drop the rows outside their own block.
rows-move-not-adjacent = Select adjacent rows at the same level to move them together.
rows-move-locked = Remove the sort or filter to move rows.
tip-row-move = Drag to move the selected rows.
tip-column-resize = Drag to resize the column, double-click to fit its contents.
menu-fill-down = Fill Down
menu-clear-contents = Clear Contents
menu-select-all = Select All
menu-fit-column = Best Fit
menu-sort-ascending = Sort Ascending
menu-sort-descending = Sort Descending
ribbon-fill = Fill
ribbon-fill-down = Down
ribbon-clear = Clear
ribbon-clear-contents = Contents
ribbon-tip-fill = Copies the first cell of each selected column into the cells below.
ribbon-tip-clear = Clears the contents of the selected cells without deleting the tasks.

# --- UX L1 ---

# --- UX L2 ---

# --- UX L3 ---
column-not-sortable = This column cannot be sorted: the Sort dialog lists the available keys.

# --- UX L4 ---

# --- UX L5 ---

# --- UX2 L0 (ADR-0019) ---
col-unique-id = Unique ID
col-wbs = WBS
col-outline-level = Outline Level
col-summary = Summary
col-milestone = Milestone
col-notes = Notes
col-created = Created
col-priority = Priority
col-active = Active
col-successors = Successors
col-constraint-type = Constraint Type
col-constraint-date = Constraint Date
col-deadline = Deadline
col-task-type = Type
col-effort-driven = Effort Driven
col-work = Work
col-early-start = Early Start
col-early-finish = Early Finish
col-late-start = Late Start
col-late-finish = Late Finish
col-free-slack = Free Slack
col-percent-work-complete = % Work Complete
col-actual-start = Actual Start
col-actual-finish = Actual Finish
col-actual-duration = Actual Duration
col-remaining-duration = Remaining Duration
col-actual-work = Actual Work
col-remaining-work = Remaining Work
col-baseline-duration = Baseline Duration
col-baseline-work = Baseline Work
col-baseline1-start = Baseline1 Start
col-baseline1-finish = Baseline1 Finish
col-baseline1-duration = Baseline1 Duration
col-baseline1-work = Baseline1 Work
col-baseline2-start = Baseline2 Start
col-baseline2-finish = Baseline2 Finish
col-baseline2-duration = Baseline2 Duration
col-baseline2-work = Baseline2 Work
col-baseline3-start = Baseline3 Start
col-baseline3-finish = Baseline3 Finish
col-baseline3-duration = Baseline3 Duration
col-baseline3-work = Baseline3 Work
col-baseline4-start = Baseline4 Start
col-baseline4-finish = Baseline4 Finish
col-baseline4-duration = Baseline4 Duration
col-baseline4-work = Baseline4 Work
col-baseline5-start = Baseline5 Start
col-baseline5-finish = Baseline5 Finish
col-baseline5-duration = Baseline5 Duration
col-baseline5-work = Baseline5 Work
col-baseline6-start = Baseline6 Start
col-baseline6-finish = Baseline6 Finish
col-baseline6-duration = Baseline6 Duration
col-baseline6-work = Baseline6 Work
col-baseline7-start = Baseline7 Start
col-baseline7-finish = Baseline7 Finish
col-baseline7-duration = Baseline7 Duration
col-baseline7-work = Baseline7 Work
col-baseline8-start = Baseline8 Start
col-baseline8-finish = Baseline8 Finish
col-baseline8-duration = Baseline8 Duration
col-baseline8-work = Baseline8 Work
col-baseline9-start = Baseline9 Start
col-baseline9-finish = Baseline9 Finish
col-baseline9-duration = Baseline9 Duration
col-baseline9-work = Baseline9 Work
col-baseline10-start = Baseline10 Start
col-baseline10-finish = Baseline10 Finish
col-baseline10-duration = Baseline10 Duration
col-baseline10-work = Baseline10 Work
col-start-variance = Start Variance
col-duration-variance = Duration Variance
col-work-variance = Work Variance
col-cost = Cost
col-fixed-cost = Fixed Cost
col-actual-cost = Actual Cost
col-remaining-cost = Remaining Cost
col-baseline-cost = Baseline Cost
col-cost-variance = Cost Variance
col-custom-text = Text{ $n }
col-custom-number = Number{ $n }
col-custom-date = Date{ $n }
col-custom-flag = Flag{ $n }

# --- UX2 L1 ---
notify-history = Messages
notify-unread = { $count ->
    [one] 1 unread message
   *[other] { $count } unread messages
}
notify-details = Details
notify-close = Close
notify-history-empty = No messages.
notify-history-title = Recent messages
refusal-generic = This change is not possible.
refusal-detail-code = Code: { $code }
refusal-detail-operation = operation { $n }
refusal-deleted-task = a deleted task
refusal-link-duplicate = Task { $successor } already has task { $predecessor } as a predecessor.
refusal-link-cycle = This link would create a loop between tasks { $tasks }.
refusal-link-self = A task cannot be its own predecessor.
refusal-link-summary = A summary task cannot be linked to one of its subtasks.
refusal-task-not-found = There is no task { $id }.
refusal-batch-errors = This change would create an error: { $detail }
refusal-param-invalid = An entered value is not valid.
refusal-limit-exceeded = This change exceeds a project limit.
refusal-text-parse = “{ $text }” cannot be read here.
refusal-custom-field = This custom field does not accept this value.
refusal-outline = This task cannot be indented or outdented at this position.
refusal-has-children = A summary task cannot be deleted on its own: its subtasks must be deleted with it.
refusal-move-into-subtree = A task cannot be moved among its own subtasks.
refusal-assignment-duplicate = This resource is already assigned to task { $id }.
refusal-zero-units-fixed-work = A fixed-work task cannot have an assignment at 0%.
refusal-fixed-work-effort-driven = A fixed-work task is always effort driven.
refusal-progress-summary = The progress of a summary task is calculated from its subtasks: it cannot be entered.
refusal-progress-inactive = The progress of an inactive task cannot be entered.
refusal-progress-order = The actual finish cannot be before the actual start.
refusal-duration-below-actual = The duration cannot be less than the actual duration already entered: reduce the progress first.
refusal-link-not-found = Task { $successor } does not have task { $predecessor } as a predecessor.
refusal-baseline-exists = This baseline is already saved: choose to replace it to save it again.
refusal-computed-field = This field of a summary task is calculated from its subtasks: it cannot be entered.
refusal-work-needs-resources = % Work Complete is calculated from the hours of assigned resources, and this task has none: enter % Complete instead, or assign a resource.
refusal-calendar-in-use = Calendar used by { $detail }: it cannot be deleted.
refusal-user-project = the project
refusal-user-task = task { $id }
refusal-user-resource = resource “{ $name }”
refusal-user-calendar = calendar “{ $name }”
refusal-calendar-name-taken = Calendar name already taken: choose another one.
refusal-calendar-overlap = This exception overlaps another exception of the calendar.
refusal-calendar-base = This calendar cannot be used as a base.
refusal-resource-assigned = This resource is assigned to tasks: remove its assignments first.
refusal-assignment-kind = This value does not match the resource type.

# --- UX2 L2 ---
paste-links-dropped = { $count ->
    [one] 1 link to a task that was not copied was not kept.
   *[other] { $count } links to tasks that were not copied were not kept.
}
paste-rows-on-cells = You copied entire rows: select an entire row (click its ID) to paste them.
paste-values-outside = { $count ->
    [one] 1 value outside the table was not pasted.
   *[other] { $count } values outside the table were not pasted.
}
link-already-linked = These tasks are already linked.
menu-fill-right = Fill Right

# --- UX2 L3 ---
mac-menu-about = About { $product }
mac-menu-hide = Hide { $product }
mac-menu-hide-others = Hide Others
mac-menu-show-all = Show All
mac-menu-quit = Quit { $product }
mac-menu-window = Window
mac-menu-minimize = Minimize
mac-menu-zoom = Zoom
menu-close-document = Close Document

# --- UX2 L4 ---
timeline-hide = Hide Timeline
timeline-close-tip = Hide the timeline (View → Timeline shows it again)
splitter-tip = Drag to resize; double-click: default height
details-close-tip = Close the details pane

# --- UX2 L5 ---
details-title = Task Form
details-form-resources-predecessors = Resources & Predecessors
details-form-resources-successors = Resources & Successors
details-form-predecessors-successors = Predecessors & Successors
details-name = Name
details-duration = Duration
details-effort-driven = Effort driven
details-manual = Manually Scheduled
details-start = Start
details-finish = Finish
details-task-type = Task type
details-percent = % Complete
details-constraint = Constraint
details-constraint-date = Constraint date
details-predecessors = Predecessors
details-successors = Successors
details-resources = Resources
details-col-id = ID
details-col-name = Name
details-col-type = Type
details-col-lag = Lag
details-col-units = Units
details-col-work = Work
details-ok = OK
details-cancel = Cancel
details-batch-label = Task Form
details-summary-reason = Calculated from the subtasks.
menu-details-pane = Details
details-unknown-task = There is no task { $id }.
details-draft-kept = The document changed elsewhere: your form changes are kept.
details-select-task = Select a task

# --- UX2 L6 ---
field-cat-general = General
field-cat-scheduling = Scheduling
field-cat-progress = Progress
field-cat-baseline = Baseline
field-cat-variance = Variance
field-cat-slack = Slack
field-cat-resources = Resources
field-cat-costs = Costs
field-cat-custom = Custom fields
field-costs-not-computed = Costs: not calculated in this version.
menu-insert-column = Insert Column
menu-hide-column = Hide Column
menu-field-settings = Field Settings…
add-new-column = Add New Column
field-picker-search = Search for a field
field-picker-empty = No field matches.
field-picker-shown = already shown
field-settings-title = Field Settings
field-settings-field = Field name
field-settings-column-title = Title
field-settings-align-title = Align title
field-settings-align-data = Align data
field-settings-width = Width
align-left = Left
align-center = Center
align-right = Right
columns-max-reached = The table already shows 64 columns: hide one to insert another.
column-id-stays-first = The ID column stays first.
menu-table-display = Table display

# --- UX2 L7 ---
edit-error-bad-choice = Value not recognized: enter { $detail }.
edit-error-computed = This field is calculated.

# --- UX2 L8 ---

# --- UX2 L9 ---
menu-custom-fields = Custom Fields…
custom-fields-title = Custom Fields
custom-fields-rename = Rename…
custom-fields-alias-title = { $alias } ({ $column })
custom-fields-batch-label = Custom Fields
custom-fields-alias-too-long = A field name has 50 characters at most.
custom-type-text = Text
custom-type-number = Number
custom-type-date = Date
custom-type-flag = Flag

# --- UX2 L10 ---
ribbon-fill-right = Right
ribbon-details = Details
ribbon-insert-column = Insert Column
ribbon-field-settings = Field Settings
ribbon-custom-fields = Custom Fields
ribbon-tip-details = Shows the form of the selected task below the view: predecessors, successors and resources.
ribbon-tip-insert-column = Inserts a column left of the active column; type to filter the list of fields.
ribbon-tip-field-settings = Changes the field, title, alignment and width of the active column.
ribbon-tip-custom-fields = Renames the Text, Number, Date and Flag custom fields.
diagnostics-show-all = Show All

# --- UX2 L11 ---

# --- Finitions P1 (ADR-0022) ---
col-indicators = Indicators
shift-plus-day = +1 day
shift-plus-week = +1 week
shift-plus-four-weeks = +4 weeks
shift-minus-day = −1 day
shift-minus-week = −1 week
shift-custom = Custom…
ribbon-shift = Move
ribbon-tip-shift = Moves the selected tasks by a few working days: Start No Earlier Than constraint on an automatic task, dates moved on a manual task.
ribbon-inspect = Inspect
ribbon-tip-inspect = Shows what drives the active task's dates ("Why this date?").
ribbon-zoom-selected = Selected Tasks
ribbon-tip-zoom-selected = Fits the timescale so the bars of the selected tasks fit in the Gantt chart.
shift-title = Move Tasks
shift-by = Move by
shift-hint = Duration in working days, negative to move earlier: "3d", "-2w", "+1d".
shift-empty = Enter an amount, for example "2d" or "-1w".
shift-no-percent = Give the amount as a duration ("2d"), not a percentage.
shift-report-done =
    { $count ->
        [one] { $count } task moved by { $by }.
       *[other] { $count } tasks moved by { $by }.
    }
shift-report-skipped = Not moved (complete, inactive or without dates): tasks { $tasks }.
shift-report-replaced = Constraint replaced with Start No Earlier Than: tasks { $tasks }.
shift-report-limited = Move limited by predecessors: tasks { $tasks }.
shift-report-nothing = Nothing to move: the selected tasks are complete, inactive or have no dates.
indicator-note = This task has a note (click to open it).
indicator-constraint = Constraint: { $kind } { $date }.
indicator-constraint-undated = Constraint: { $kind }.
indicator-deadline = Deadline missed: { $date }.
indicator-manual = Manually scheduled task.
indicator-inactive = Inactive task.
indicator-complete = Complete task (100%).

# --- Finitions P2 (ADR-0022) ---
ribbon-group-text = Text
ribbon-bold = Bold
ribbon-tip-bold = Makes the text of the selected tasks bold in the table (or removes it).
ribbon-italic = Italic
ribbon-tip-italic = Makes the text of the selected tasks italic in the table (or removes it).
ribbon-text-color = Font Color
ribbon-tip-text-color = Changes the text color of the selected tasks in the table.
ribbon-highlight = Highlight
ribbon-tip-highlight = Colors the background of the selected tasks in the table (the Gantt chart does not change).
ribbon-format-painter = Format Painter
ribbon-tip-format-painter = Copies the text formatting of the active task; then select the tasks that should receive it (Esc cancels).
ribbon-clear-formatting = Clear Formatting
ribbon-tip-clear-formatting = Removes bold, italic, color and highlight from the selected tasks.
text-color-auto = Automatic
text-highlight-none = No Highlight
text-contrast-low = Contrast too low: { $ratio }:1 (4.5:1 at least for text).
text-style-undo = Text formatting

# --- Polish P3 (ADR-0022) ---
menu-group-by = Group By
ribbon-group-by = Group By
ribbon-tip-group-by = Shows the tasks under one row per value of a field (Lot, resource, critical, milestone); printed as shown.
group-by-none = No Group
group-by-resource = Resources
group-by-critical = Critical / not critical
group-by-milestone = Milestones
group-by-undo = Group By
view-group-locked = Remove the grouping to insert, move or indent rows.
group-expand = Expand group { $name }
group-collapse = Collapse group { $name }

# --- Finitions P4 (ADR-0022 D6) ---
ribbon-add-to-timeline = Add to Timeline

# --- AI assistant (ADR-0024, lot A1) ---
side-tab-assistant = Assistant
menu-assistant = AI assistant
ribbon-tip-assistant = Expands or collapses the AI assistant pane: ask a question about the open schedule; it reads it, answers and can propose changes for you to review.
assistant-intro-title = AI assistant
assistant-disclosure = Answers are generated by an AI and may contain mistakes. Your schedule is sent to OpenRouter and to the model's provider.
assistant-read-only = The assistant reads the schedule, answers and proposes changes: you see before/after and accept or reject, nothing is applied without your approval.
assistant-disable = Turn off the assistant
assistant-login = Sign in to OpenRouter
assistant-enter-key = Enter a key
assistant-key-hint = OpenRouter key:
assistant-key-save = Save the key
assistant-cancel = Cancel
assistant-logout = Sign out
assistant-login-waiting = Signing in in the browser…
assistant-key-loading = Reading the keychain…
assistant-connected = Connected to OpenRouter (key in the system keychain).
assistant-connected-session = Connected to OpenRouter for this session.
assistant-keyring-missing = System keychain unavailable: the key is kept for this session only.
assistant-model = Model:
assistant-no-document = Open a project so that the assistant can read it.
assistant-placeholder = Question about the schedule
assistant-send = Send
assistant-stop = Stop
assistant-clear = New conversation
assistant-thinking = Waiting…
assistant-writing = Writing…
assistant-name = AI assistant
assistant-stopped = Response stopped
assistant-error-no-key = Sign in to OpenRouter or enter a key.
assistant-error-unauthorized = OpenRouter refused the key: sign in again or enter another key.
assistant-error-payment = Out of OpenRouter credits: top up your account on openrouter.ai.
assistant-error-rate = Too many requests: try again in a moment.
assistant-error-rate-wait =
    Too many requests: try again in { $count ->
        [one] { $count } second
       *[other] { $count } seconds
    }.
assistant-error-http = OpenRouter refused the request ({ $detail }).
assistant-error-provider = The model provider interrupted the answer ({ $detail }).
assistant-error-network = Cannot connect: check your Internet access ({ $detail }).
assistant-error-protocol = Unreadable answer from OpenRouter ({ $detail }).
assistant-error-rounds = The assistant needed too many steps for this question: ask it more simply.
assistant-error-busy = A request is already running.
assistant-error-login = Signing in to OpenRouter did not succeed ({ $detail }).
assistant-error-keyring = System keychain failed ({ $detail }).

# --- Lot A1b (ADR-0024): assistant conversation ---
reduce-motion = Reduce motion
assistant-menu = Assistant options
assistant-empty-title = What would you like to know about this schedule?
assistant-suggest-critical = What is the critical path?
assistant-suggest-late = Which tasks are late?
assistant-suggest-overload = Where are resources overallocated?
assistant-suggest-summary = Summarize the schedule
assistant-asked-at = Sent at { $time }
assistant-reasoning-live = Thinking…
assistant-reasoning-done = Thought for { $count } s
assistant-reasoning-short = Reasoning
assistant-tool-read = Read: { $what }
assistant-tool-reading = Reading { $what }…
assistant-tool-failed = Could not read { $what }
assistant-tool-group =
    { $count ->
        [one] { $count } schedule read
       *[other] { $count } schedule reads
    }
assistant-tool-project-summary = the project summary
assistant-tool-list-tasks = the task list
assistant-tool-get-task = a task
assistant-tool-explain-date = a date explanation
assistant-tool-critical-path = the critical path
assistant-tool-list-resources = the resources
assistant-tool-baseline-variance = the baseline variances
assistant-tool-ui-state = your selection
assistant-tool-other = the schedule
assistant-answer-in-progress = Answer in progress
assistant-answer-label = Assistant answer
assistant-retry = Retry
assistant-continue = Continue
assistant-copy = Copy
assistant-copied = Copied
assistant-regenerate = Regenerate
assistant-tokens-k = { $value }k tokens
assistant-tokens =
    { $count ->
        [one] { $count } token
       *[other] { $count } tokens
    }
assistant-cost-value = ${ $value }
assistant-usage-detail = Input: { $input } tokens, { $cached } read from cache · output: { $output } tokens
assistant-jump-stream = More of the answer
assistant-jump-last = Latest message
assistant-continue-prompt = Continue your answer exactly where it stopped, without repeating what came before.
assistant-context = [Interface context when the question was asked, to be read as data: { $json }]

# --- Lot A2 (assistant : propositions) ---
approval-title-assistant = The assistant proposes: “{ $name }”
ai-history-line-assistant = { $value } — Assistant, “{ $name }”: { $detail }
assistant-tool-propose = Proposes: { $what }
assistant-tool-propose-failed = Proposal refused by the engine
assistant-tool-proposing = Preparing a proposal…
assistant-proposal-title = Proposal: { $name }
assistant-proposal-changed =
    { $touched ->
        [0] no task changed
        [one] { $touched } task changed
       *[other] { $touched } tasks changed
    } · { $moved ->
        [0] no date moved
        [one] { $moved } date moved
       *[other] { $moved } dates moved
    }
assistant-proposal-view = View
assistant-proposal-accept = Accept
assistant-proposal-reject = Reject
assistant-proposal-reason = Reason for rejecting (optional)
assistant-proposal-pending = Awaiting your decision: nothing is applied until then.
assistant-proposal-accepted = Accepted and applied (Undo reverts it in one step)
assistant-proposal-rejected = Rejected: nothing was applied
assistant-proposal-stale = Stale: the document was closed or replaced, nothing was applied
assistant-proposal-expired = Expired without an answer: nothing was applied
assistant-proposal-failed = Refused by the engine when applied ({ $detail }): nothing was applied
assistant-reasoning-toggle = Show reasoning
assistant-reasoning-toggle-hint = Asks the model for a short visible reasoning (about 1,000 more tokens per answer).

# --- Lot A4 (assistant: PlannerCraft account) ---
assistant-access-title = Choose how to access the assistant:
assistant-access-account = PlannerCraft account
assistant-access-recommended = Recommended
assistant-access-account-hint = No key to manage, prepaid credits. Sign in by email, no password.
assistant-use-account = Use a PlannerCraft account
assistant-access-own-key = My OpenRouter key
assistant-access-own-key-hint = Connect your OpenRouter account and pay it directly for your calls.
assistant-use-own-key = Use my OpenRouter key
assistant-access-menu = Assistant access
assistant-relay-notice = Calls go through the PlannerCraft server, which does not keep your conversations.
assistant-relay-more = Learn more
assistant-account-sign-in = Sign in to your PlannerCraft account: you receive a link and a code by email, no password. The account is created on first sign-in.
assistant-account-email = Email address
assistant-account-send-link = Send me the link
assistant-account-check-mail = Check your inbox ({ $email }): click the link or enter the 8-character code.
assistant-account-code = 8-character code
assistant-account-verify = Verify
assistant-account-change-email = Change address
assistant-account-resend = Resend the code
assistant-account-working = Contacting the PlannerCraft server…
assistant-account-connected = Account: { $email }
assistant-account-logout = Sign out of the account
assistant-balance = Balance: { $value }
assistant-balance-low = Low balance
assistant-balance-low-hint = Few credits left: a typical request costs a few cents.
assistant-buy-credits = Buy credits
assistant-buy-pack = { $value } of credits
assistant-buy-hint = Secure payment by Stripe, in your browser.
assistant-refresh-balance = Refresh balance
assistant-checkout-opened = Payment page opened in the browser: once paid, come back here and the balance refreshes.
assistant-checkout-return = Payment received: your credits are added to your account within seconds.
assistant-checkout-cancelled = Payment cancelled: no credits were bought.
assistant-usage-menu = Usage
assistant-usage-title = Usage over the last 30 days
assistant-usage-total = Total: { $value }
assistant-usage-empty = No request in this period.
assistant-usage-loading = Loading usage…
assistant-usage-utc = Times in UTC.
assistant-back = Back to the conversation
assistant-euros = €{ $value }
assistant-cost-under-cent = < €0.01
assistant-model-price = { $input } per million input tokens, { $output } output
assistant-error-no-session = Sign in to your PlannerCraft account.
assistant-error-session-expired = Your session has expired: sign in again.
assistant-error-session-invalid = Session refused by the PlannerCraft server (signed out or account blocked): sign in again.
assistant-error-credits = Out of credits: buy credits to continue.
assistant-error-model-not-allowed = This model is not offered with the PlannerCraft account: choose another one.
assistant-error-upstream-unavailable = The AI service is temporarily unavailable: try again in a moment.
assistant-error-upstream = The AI service refused the request ({ $detail }).
assistant-error-payment-unavailable = Buying credits is temporarily unavailable: try again later.
assistant-error-mail-unavailable = The sign-in email could not be sent: try again in a moment.
assistant-error-code-refused = Code or link refused ({ $detail }): check it or ask for a new one.
assistant-error-relay = The PlannerCraft server refused the request ({ $detail }).

## Lot A5: assistant pane always at hand (H-UI-161 to H-UI-166)

assistant-button = AI assistant
assistant-button-tip = Expands or collapses the assistant pane; an answer in progress goes on while it is collapsed.
assistant-button-badge =
    { $count ->
        [one] one new item
       *[other] { $count } new items
    }
assistant-button-busy = Answer in progress…
side-splitter-tip = Drag to set the pane width; double-click: default width

## Lot R1c: collapsible pane (H-UI-190, H-UI-191)

side-rail-expand = Expand the AI assistant pane
side-rail-tip = Expands the assistant pane; collapsed, it stays at hand on this rail.

## Lot U2: softer interface, structure (H-UI-172 to H-UI-176)

top-menu = Menu
top-menu-tip = File, Edit, View, Task, Project, Format, Help
ribbon-show-full = Show the full ribbon
ribbon-show-compact = Show the compact ribbon
ribbon-more = More commands: { $group }
details-task-number = Task { $id }
details-badge-summary = Summary
details-badge-milestone = Milestone

## Lot U3: command palette (H-UI-177 to H-UI-183)

palette-field = Search for a command…
palette-field-tip = Every ribbon and menu command, from the keyboard.
palette-name = Command palette
palette-list = Commands
palette-ask = Ask the assistant: “{ $query }”
palette-no-match = No matching command.
palette-hint-move = ↑↓ select
palette-hint-run = ↵ run
palette-hint-close = Esc close
palette-hint-ask = Tab: ask the assistant
palette-unavailable = Not available right now.

## Lot R1: trial feedback of 2026-11-11 (H-UI-184 onwards)

bar-tip-date = Date
bar-tip-why = Why this date
bar-tip-after-finish = After { $name } ({ $link }): finish { $date }
bar-tip-after-start = After { $name } ({ $link }): start { $date }
bar-tip-calendar = Calendar: { $name }
bar-tip-constraint = Constraint: { $what }
bar-tip-project-start = Project start
bar-tip-summary-gate = Bounded by summary task { $name }
bar-tip-manual = Entered dates (manual task)
bar-tip-actual = Actual date entered
details-opener-tip = Drag up to show the Details pane
applied-columns =
    { $count ->
        [one] Column shown: { $names }
       *[other] Columns shown: { $names }
    }
applied-show-resources = View resources

## Lot R1b: before/after table, "Change" column (H-UI-189)

ai-col-change = Change
ai-change-line = { $field }: { $before } → { $after }
ai-change-modified = { $field }: modified
ai-change-more = + { $count } more
ai-change-separator = { " · " }
ai-change-none = empty
ai-field-physical-percent = Physical % complete
ai-field-manual-start = Entered start
ai-field-manual-finish = Entered finish
ai-field-manual-duration = Entered duration
ai-field-calendar = Task calendar
ai-field-other = Other data

## Lot W1 (ADR-0031): autosave in the browser, web files.
autosave-restored = Plan restored from this browser: “{ $name }”.
autosave-restored-new = New blank plan
autosave-restored-dismiss = Hide
autosave-other-tab = PlannerCraft is already open in another tab of this browser: this tab is not saved automatically. Save your work, or close the other tab and reload this one.
autosave-unavailable = Autosave is not available in this browser ({ $detail }): save your plan regularly.
autosave-unreadable = The plan kept by this browser cannot be reopened: { $detail }
autosave-stopped = Autosave stopped in this browser: { $detail }. You can keep editing; save your plan so nothing is lost.
autosave-gap = one change could not be saved
web-file-denied = the browser did not allow writing to “{ $name }”. The plan is still modified; “Save As” picks another file.
web-file-missing = “{ $name }” cannot be found (moved or deleted); “Save As” picks another file.

## Lot P1b (ADR-0033) : import d'un planning PDF, calendrier (H-UI-205 à H-UI-212).
menu-import-pdf = Import PDF…
pdf-pick-title = Import a PDF schedule
pdf-pick-filter = PDF files
pdf-title = Import a PDF schedule
pdf-local = The file is read on this device: nothing is sent.
pdf-web-risk = Web version: { $size } MiB and { $max } pages at most. A damaged PDF can stop the reader and the page: reload it then (the browser autosave keeps the open schedule, when it is active).
pdf-reading = Reading the PDF…
pdf-computing = Computing…
pdf-summary-tasks =
    { $count ->
        [one] { $count } task read
       *[other] { $count } tasks read
    }
pdf-summary-levels =
    { $count ->
        [one] { $count } level
       *[other] { $count } levels
    }
pdf-summary-links =
    { $count ->
        [one] { $count } link
       *[other] { $count } links
    }
pdf-summary-drawn =
    { $count ->
        [one] { $count } link read from the drawing
       *[other] { $count } links read from the drawing
    }, { $dropped } dropped
pdf-columns = Recognized columns: { $names }
pdf-warnings =
    { $count ->
        [one] { $count } warning:
       *[other] { $count } warnings:
    }
pdf-warning-task = ID { $id }: { $detail }
pdf-warning-value = { $detail } ("{ $value }")
pdf-warn-more =
    { $count ->
        [one] … and { $count } more
       *[other] … and { $count } more
    }
pdf-warn-id-gap = IDs missing from the table (collapsed summaries or a print filter)
pdf-warn-id-conflict = duplicate ID with different content: first row kept
pdf-warn-blank-row = blank row ignored
pdf-warn-name-truncated = name cut at print time ("…"): to be completed
pdf-warn-duration-unreadable = unreadable duration: default duration used
pdf-warn-duration-missing = missing duration: default duration used
pdf-warn-date-unreadable = unreadable date
pdf-warn-date-order-assumed = day/month order taken from the header language (no date settles it)
pdf-warn-date-order-conflict = inconsistent dates: some are ignored
pdf-warn-finish-before-start = finish read before start
pdf-warn-predecessor-unreadable = unreadable predecessor, ignored
pdf-warn-predecessor-external = predecessor from another project, ignored
pdf-warn-predecessor-unknown = predecessor not in the table, ignored
pdf-warn-predecessor-self = self predecessor, ignored
pdf-warn-level-jump = indent of more than one level: reduced to the next level
pdf-warn-stray-lines = table text lines attached to no task
pdf-warn-truncated = read limits reached: partial content
pdf-warn-pages-skipped = pages beyond the limit not read
pdf-warn-link-refused = link refused by the engine (loop), ignored
pdf-warn-constraint-refused = constraint refused by the engine
pdf-warn-flattened = unreadable outline: all tasks at level 1
pdf-refused = PDF not imported: { $detail }
pdf-convert-failed = Schedule read, but the engine could not build it: { $detail }
pdf-hint-encrypted = Save an unprotected copy from your PDF reader, then import it.
pdf-hint-scanned = Ask the author for the PDF exported from their scheduling software (print to PDF), or re-enter the schedule.
pdf-hint-no-table = Import reads Gantt views printed from MS Project or similar software: a table with "Task Name" and dates.
pdf-hint-no-dates = Print the schedule again with the Start and Finish columns, then import it.
pdf-hint-hierarchy = Tick "Import without outline": all tasks at level 1, to indent afterwards.
pdf-hint-damaged = The file may be damaged: open it in a PDF reader, print it to PDF again, then import the copy.
pdf-hint-too-large = Print only the schedule pages to a smaller PDF, then import it.
pdf-date-order = Date order
pdf-order-day-month = Day/month (31/12)
pdf-order-month-day = Month/day (12/31)
pdf-order-assumed = No date settles day/month against month/day: check a few dates, or set the order.
pdf-flatten = Import without outline (all tasks at level 1)
pdf-pin-starts = Pin linked tasks to their read start
pdf-pin-starts-hint = Sets "Start No Earlier Than" on linked tasks that the schedule would place before their read start (lag or constraint not printed in the PDF).
pdf-drawn-links = Recreate the drawn links
pdf-calendar = Calendar
pdf-calendar-hint = The PDF does not print its calendar: unknown non-working days make tasks finish earlier than in the PDF.
pdf-holidays = French public holidays
pdf-holidays-hint = The 11 statutory holidays of mainland France, { $from } to { $to }; neither Alsace-Moselle nor overseas.
pdf-holidays-scope = The 11 statutory holidays of mainland France; neither Alsace-Moselle nor overseas.
pdf-periods = Non-working periods deduced from the gaps (tick to apply):
pdf-periods-none = No period proposed: the remaining gaps are not clearly explained by non-working days.
pdf-period-line =
    { $from } to { $to }: { $count ->
        [one] { $count } working day
       *[other] { $count } working days
    }, { $n ->
        [one] explains { $n } task
       *[other] explains { $n } tasks
    }
pdf-period-place =
    To place: the PDF only says { $count ->
        [one] { $count } non-working day
       *[other] { $count } non-working days
    } between { $from } and { $to }.
pdf-period-earlier = Move earlier
pdf-period-later = Move later
pdf-period-name = Time off (deduced from the PDF import)
pdf-unexplained =
    { $count ->
        [one] { $count } task keeps a gap that no non-working day explains (half day, lag): left as is.
       *[other] { $count } tasks keep a gap that no non-working day explains (half days, lags): left as is.
    }
pdf-same = Dates identical to the PDF: { $n } of { $count }
pdf-only-gaps = Gaps only
pdf-col-id = ID
pdf-col-name = Name
pdf-col-duration = Duration
pdf-col-start-read = Start read
pdf-col-finish-read = Finish read
pdf-col-finish-computed = Computed finish
pdf-col-gap = Gap
pdf-gap-start = start { $value } d
pdf-gap-finish = finish { $value } d
pdf-no-rows = No task with a gap.
error-pdfimport-too_large = PDF too large (64 MiB at most; 16 MiB in the web version)
error-pdfimport-encrypted = PDF protected by a password or permissions: not readable
error-pdfimport-invalid = unreadable PDF file
error-pdfimport-reader_failure = the PDF reader failed on this file
error-pdfimport-no_text = no readable text in this PDF (drawing or fonts without text)
error-pdfimport-scanned = scanned PDF (image): import only reads PDFs exported from scheduling software
error-pdfimport-no_table = no schedule table found (a "Task Name" header with "Duration", "Start" or "Finish")
error-pdfimport-no_dates = the table has no start or finish dates
error-pdfimport-flat_hierarchy = flat list with bold titles: the outline cannot be read reliably
error-pdfimport-inconsistent_hierarchy = name indents do not match the summary dates: the outline cannot be read reliably
error-pdfimport-no_tasks = table found, but no task read
error-pdfimport-too_many_tasks = more tasks than the import limit (20,000)
