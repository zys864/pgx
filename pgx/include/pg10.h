#include <stdbool.h>
#define USE_STDBOOL 1

#include "postgres.h"
#include "miscadmin.h"
#include "access/amapi.h"
#include "access/genam.h"
#include "access/htup.h"
#include "access/htup_details.h"
#include "access/relscan.h"
#include "access/skey.h"

#define ScanKey struct ScanKeyData *
#include "nodes/execnodes.h"
#undef ScanKey

#include "catalog/index.h"
#include "catalog/pg_type.h"
#include "commands/event_trigger.h"
#include "executor/executor.h"
#include "executor/spi.h"
#include "nodes/execnodes.h"
#include "nodes/extensible.h"
#include "nodes/makefuncs.h"
#include "nodes/nodes.h"
#include "nodes/replnodes.h"
#include "nodes/tidbitmap.h"
#include "nodes/value.h"
#include "optimizer/planner.h"
#include "storage/itemptr.h"
#include "utils/builtins.h"
#include "utils/jsonb.h"
#include "utils/memutils.h"
#include "utils/palloc.h"
#include "utils/rel.h"
#include "utils/relcache.h"
