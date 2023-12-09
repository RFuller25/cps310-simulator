#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct RArmSimKernel RArmSimKernel;

typedef struct RArmSimKernel *OwningHandle;

typedef const struct RArmSimKernel *BorrowHandle;

typedef struct RArmSimKernel *BorrowMutHandle;

OwningHandle rask_init(const ask_host_services_t *hostp);

void rask_fini(OwningHandle _h);

ask_config_t rask_config_get(BorrowHandle h);

void rask_config_set(BorrowMutHandle h, ask_config_t flags);

void rask_stats_report(BorrowHandle h, ask_stat_t *output);

uint32_t rask_reg_get(BorrowHandle h, ask_mode_t _bank, int32_t index);

void rask_reg_set(BorrowMutHandle h, ask_mode_t _bank, int32_t index, uint32_t value);

uint32_t rask_cpsr_get(BorrowHandle h);

void rask_cpsr_set(BorrowMutHandle h, uint32_t value);

int32_t rask_cpu_running(BorrowHandle _h);

void rask_cpu_signal(BorrowMutHandle _h, ask_signal_t _signal);

int32_t rask_cpu_run(BorrowMutHandle h, int32_t cycles);
