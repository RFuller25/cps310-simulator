/* TODO: provide your name and project information here */
#include <stddef.h>

/* ArmSim Kernel/Shell Interface definition */
#include "askapi.h"

/* Rust library bindings for calling into actual kernel implementation */
#include "rust/bindings.h"

static char *info_strings[] = {
	"author=rfull917",
	"api=1.0",
	"prototype",
	NULL};

// Global handle to the created kernel
OwningHandle rask = NULL;

char **ask_info(void)
{
	return info_strings;
}

void ask_disasm(word address, word instruction, char *buff, size_t size)
{
	// TODO
}

void ask_init(const struct ask_host_services *host)
{
	if (rask)
		rask_fini(rask);
	rask = rask_init(host);
	for (int i = 0; i < 11; i++)
	{
		rask_reg_set(rask, 0, i, 0);
	}
	// Uncomment to activate rough design test on init
}

void ask_config_set(ask_config_t flags)
{
	rask_config_set(rask, flags);
}

ask_config_t ask_config_get(void)
{
	return rask_config_get(rask);
}

void ask_stats_report(struct ask_stats *stats)
{
	rask_stats_report(rask, stats);
}

word ask_reg_get(ask_mode_t bank, int index)
{
	return rask_reg_get(rask, bank, index);
}

void ask_reg_set(ask_mode_t bank, int index, word value)
{
	rask_reg_set(rask, bank, index, value);
}

word ask_cpsr_get(void)
{
	return rask_cpsr_get(rask);
}

void ask_cpsr_set(word value)
{
	return rask_cpsr_set(rask, value);
}

int ask_cpu_running(void)
{
	return rask_cpu_running(rask);
}

void ask_cpu_signal(ask_signal_t signal)
{
	rask_cpu_signal(rask, signal);
}

int ask_cpu_run(int cycles)
{
	return rask_cpu_run(rask, cycles);
}

word shut_up_and_sub(word a, word b, int *cf) {
        *cf = (a >= b) ? 1 : 0;
        return a - b;
}


word shut_up_and_add(word a, word b) {
        return a + b;
}