// go:build ignore

#include "headers/common.h"

char __license[] SEC("license") = "GPL";

struct bpf_map_def SEC("maps") events = {
    .type = BPF_MAP_TYPE_PERF_EVENT_ARRAY,
};

// This struct is defined according to the following format file:
// /sys/kernel/debug/tracing/events/syscalls/sys_enter_execve/format
struct execve_args {
  short common_type;
  char common_flags;
  char common_preempt_count;
  int common_pid;
  int __syscall_nr;
  char *filename;
  const char *const *argv;
  const char *const *envp;
};

struct event {
  int pid;
  int uid;
  char payload[200];
};

SEC("tracepoint/syscalls/sys_enter_execve")
int sys_enter_execve(struct execve_args *ctx) {
  // Reserve a pointer to the first env var
  char *ctx_var;

  int i = 0;
  long res;
  struct event event = {0};
  while (i < 10) {
    // Attempt to read the value pointed by ctx->envp[0] and store it in
    // *ctx_var
    res = bpf_probe_read(&ctx_var, sizeof(ctx_var), &ctx->argv[i]);

    if (res != 0) {
      return 0;
    }

    // Read the value pointed by the (now) safe pointer *ctx_var
    // and store the value in 'value'
    bpf_probe_read_str(event.payload, sizeof(event.payload), ctx_var);

    // Check if no string data is available
    if (!*event.payload) {
      return 0;
    }

    event.pid = bpf_get_current_pid_tgid() >> 32;
    event.uid = (u32)bpf_get_current_uid_gid();

    // Write data to the userspace application
    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &event,
                          sizeof(event));
    // Clear value buffer
    __builtin_memset(event.payload, 0, sizeof(event.payload));
    i++;
  }

  return 0;
}
