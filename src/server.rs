use std::sync::{Mutex, MutexGuard};

use lazy_static::lazy_static;
use libc::{localtime_r, timeval, tm};

use crate::crc64::crc64_init;
use crate::lib::dict::{dict_set_hash_function_seed, DictTypeTrait};
use crate::lib::time::gettimeofday;
use crate::redis_server::RedisServer;
use crate::util::get_random_bytes;

mod crc64;
mod crcspeed;
mod redis_server;
mod util;
mod lib;
mod config;


lazy_static! {
    static ref SERVER: Mutex<RedisServer> = Mutex::new(RedisServer::new());
}
const CONFIG_DEFAULT_BINDADDR_COUNT: usize = 2;
static CONFIG_DEFAULT_BINDADDR: [&str; 2] = ["*", "-::*"];


/* Return the UNIX time in microseconds */
fn ustime()-> i64 {
    let mut tv = gettimeofday(None);
    return tv.timestamp_nanos_opt().unwrap();
}

// fn dictSdsCaseHash<T>(key: &T) -> u64 {
//     return dictGenCaseHashFunction((unsigned char*)key, sdslen((char*)key));
// }

/* We take a cached value of the unix time in the global state because with
 * virtual memory and aging there is to store the current time in objects at
 * every object access, and accuracy is not needed. To access a global var is
 * a lot faster than calling time(NULL).
 *
 * This function should be fast because it is called at every command execution
 * in call(), so it is possible to decide if to update the daylight saving
 * info or not using the 'update_daylight_info' argument. Normally we update
 * such info only when calling this function from serverCron() but not when
 * calling it from call(). */
fn update_cached_time(server: &mut MutexGuard<RedisServer>, update_daylight_info: bool) {
    let mut tv = gettimeofday(None);
    let ustime =  tv.timestamp_nanos_opt().unwrap();
    server.ustime = ustime;
    server.mstime = server.ustime / 1000;
    server.unixtime = server.mstime / 1000;

    /* To get information about daylight saving time, we need to call
     * localtime_r and cache the result. However calling localtime_r in this
     * context is safe since we will never fork() while here, in the main
     * thread. The logging function will call a thread safe version of
     * localtime that has no locks. */
    unsafe {
        if update_daylight_info {
            let mut tm: tm = tm {
                tm_sec: 0,
                tm_min: 0,
                tm_hour: 0,
                tm_mday: 0,
                tm_mon: 0,
                tm_year: 0,
                tm_wday: 0,
                tm_yday: 0,
                tm_isdst: 0,
                tm_gmtoff: 0,
                tm_zone: std::ptr::null_mut(),
            };
            *localtime_r(&server.unixtime, &mut tm);
            server.daylight_active = tm.tm_isdst;
        }
    }
}
fn init_server_config(server: &mut MutexGuard<RedisServer>) {
    // int j;
    // let default_bindaddr: [&str; CONFIG_DEFAULT_BINDADDR_COUNT] = CONFIG_DEFAULT_BINDADDR;
    // initConfigValues();
    update_cached_time(server, true);
    server.cmd_time_snapshot = server.mstime;
    // getRandomHexChars(server.runid,CONFIG_RUN_ID_SIZE);
    // server.runid[CONFIG_RUN_ID_SIZE] = '\0';
    // changeReplicationId();
    // clearReplicationId2();
    // server.hz = CONFIG_DEFAULT_HZ; /* Initialize it ASAP, even if it may get
    //                                   updated later after loading the config.
    //                                   This value may be used before the server
    //                                   is initialized. */
    // server.timezone = getTimeZone(); /* Initialized by tzset(). */
    // server.configfile = NULL;
    // server.executable = NULL;
    // server.arch_bits = (sizeof(long) == 8) ? 64 : 32;
    // server.bindaddr_count = CONFIG_DEFAULT_BINDADDR_COUNT;
    // for (j = 0; j < CONFIG_DEFAULT_BINDADDR_COUNT; j++)
    //     server.bindaddr[j] = zstrdup(default_bindaddr[j]);
    // memset(server.listeners, 0x00, sizeof(server.listeners));
    // server.active_expire_enabled = 1;
    // server.lazy_expire_disabled = 0;
    // server.skip_checksum_validation = 0;
    // server.loading = 0;
    // server.async_loading = 0;
    // server.loading_rdb_used_mem = 0;
    // server.aof_state = AOF_OFF;
    // server.aof_rewrite_base_size = 0;
    // server.aof_rewrite_scheduled = 0;
    // server.aof_flush_sleep = 0;
    // server.aof_last_fsync = time(NULL) * 1000;
    // server.aof_cur_timestamp = 0;
    // atomicSet(server.aof_bio_fsync_status,C_OK);
    // server.aof_rewrite_time_last = -1;
    // server.aof_rewrite_time_start = -1;
    // server.aof_lastbgrewrite_status = C_OK;
    // server.aof_delayed_fsync = 0;
    // server.aof_fd = -1;
    // server.aof_selected_db = -1; /* Make sure the first time will not match */
    // server.aof_flush_postponed_start = 0;
    // server.aof_last_incr_size = 0;
    // server.aof_last_incr_fsync_offset = 0;
    // server.active_defrag_running = 0;
    // server.active_defrag_configuration_changed = 0;
    // server.notify_keyspace_events = 0;
    // server.blocked_clients = 0;
    // memset(server.blocked_clients_by_type,0,
    //        sizeof(server.blocked_clients_by_type));
    // server.shutdown_asap = 0;
    // server.shutdown_flags = 0;
    // server.shutdown_mstime = 0;
    // server.cluster_module_flags = CLUSTER_MODULE_FLAG_NONE;
    // server.migrate_cached_sockets = dictCreate(&migrateCacheDictType);
    // server.next_client_id = 1; /* Client IDs, start from 1 .*/
    // server.page_size = sysconf(_SC_PAGESIZE);
    // server.pause_cron = 0;
    // server.dict_resizing = 1;
    //
    // server.latency_tracking_info_percentiles_len = 3;
    // server.latency_tracking_info_percentiles = zmalloc(sizeof(double)*(server.latency_tracking_info_percentiles_len));
    // server.latency_tracking_info_percentiles[0] = 50.0;  /* p50 */
    // server.latency_tracking_info_percentiles[1] = 99.0;  /* p99 */
    // server.latency_tracking_info_percentiles[2] = 99.9;  /* p999 */
    //
    // server.lruclock = getLRUClock();
    // resetServerSaveParams();
    //
    // appendServerSaveParams(60*60,1);  /* save after 1 hour and 1 change */
    // appendServerSaveParams(300,100);  /* save after 5 minutes and 100 changes */
    // appendServerSaveParams(60,10000); /* save after 1 minute and 10000 changes */
    //
    // /* Replication related */
    // server.masterhost = NULL;
    // server.masterport = 6379;
    // server.master = NULL;
    // server.cached_master = NULL;
    // server.master_initial_offset = -1;
    // server.repl_state = REPL_STATE_NONE;
    // server.repl_transfer_tmpfile = NULL;
    // server.repl_transfer_fd = -1;
    // server.repl_transfer_s = NULL;
    // server.repl_syncio_timeout = CONFIG_REPL_SYNCIO_TIMEOUT;
    // server.repl_down_since = 0; /* Never connected, repl is down since EVER. */
    // server.master_repl_offset = 0;
    // server.fsynced_reploff_pending = 0;
    //
    // /* Replication partial resync backlog */
    // server.repl_backlog = NULL;
    // server.repl_no_slaves_since = time(NULL);
    //
    // /* Failover related */
    // server.failover_end_time = 0;
    // server.force_failover = 0;
    // server.target_replica_host = NULL;
    // server.target_replica_port = 0;
    // server.failover_state = NO_FAILOVER;
    //
    // /* Client output buffer limits */
    // for (j = 0; j < CLIENT_TYPE_OBUF_COUNT; j++)
    //     server.client_obuf_limits[j] = clientBufferLimitsDefaults[j];
    //
    // /* Linux OOM Score config */
    // for (j = 0; j < CONFIG_OOM_COUNT; j++)
    //     server.oom_score_adj_values[j] = configOOMScoreAdjValuesDefaults[j];
    //
    // /* Double constants initialization */
    // R_Zero = 0.0;
    // R_PosInf = 1.0/R_Zero;
    // R_NegInf = -1.0/R_Zero;
    // R_Nan = R_Zero/R_Zero;
    //
    // /* Command table -- we initialize it here as it is part of the
    //  * initial configuration, since command names may be changed via
    //  * redis.conf using the rename-command directive. */
    // server.commands = dictCreate(&commandTableDictType);
    // server.orig_commands = dictCreate(&commandTableDictType);
    // populateCommandTable();
    //
    // /* Debugging */
    // server.watchdog_period = 0;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let tv = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let j: i32;
    let config_from_stdin: i8 = 0;
    //
    // #ifdef REDIS_TEST
    // monotonicInit(); /* Required for dict tests, that are relying on monotime during dict rehashing. */
    // if (argc >= 3 && !strcasecmp(argv[1], "test")) {
    //     int flags = 0;
    //     for (j = 3; j < argc; j++) {
    //         char *arg = argv[j];
    //         if (!strcasecmp(arg, "--accurate")) flags |= REDIS_TEST_ACCURATE;
    //         else if (!strcasecmp(arg, "--large-memory")) flags |= REDIS_TEST_LARGE_MEMORY;
    //         else if (!strcasecmp(arg, "--valgrind")) flags |= REDIS_TEST_VALGRIND;
    //     }
    //
    //     if (!strcasecmp(argv[2], "all")) {
    //         int numtests = sizeof(redisTests)/sizeof(struct redisTest);
    //         for (j = 0; j < numtests; j++) {
    //             redisTests[j].failed = (redisTests[j].proc(argc,argv,flags) != 0);
    //         }
    //
    //         /* Report tests result */
    //         int failed_num = 0;
    //         for (j = 0; j < numtests; j++) {
    //             if (redisTests[j].failed) {
    //                 failed_num++;
    //                 printf("[failed] Test - %s\n", redisTests[j].name);
    //             } else {
    //                 printf("[ok] Test - %s\n", redisTests[j].name);
    //             }
    //         }
    //
    //         printf("%d tests, %d passed, %d failed\n", numtests,
    //                numtests-failed_num, failed_num);
    //
    //         return failed_num == 0 ? 0 : 1;
    //     } else {
    //         redisTestProc *proc = getTestProcByName(argv[2]);
    //         if (!proc) return -1; /* test not found */
    //         return proc(argc,argv,flags);
    //     }
    //
    //     return 0;
    // }
    // #endif
    //
    // /* We need to initialize our libraries, and the server configuration. */
    // #ifdef INIT_SETPROCTITLE_REPLACEMENT
    // spt_init(argc, argv);
    // #endif
    // tzset(); /* Populates 'timezone' global. */

    /* To achieve entropy, in case of containers, their time() and getpid() can
     * be the same. But value of tv_usec is fast enough to make the difference */
    // gettimeofday(&tv,NULL);
    // srand(time(NULL)^getpid()^tv.tv_usec);
    // srandom(time(NULL)^getpid()^tv.tv_usec);
    // init_genrand64(((long long) tv.tv_sec * 1000000 + tv.tv_usec) ^ getpid());
    crc64_init();

    /* Store umask value. Because umask(2) only offers a set-and-get API we have
     * to reset it and restore it back. We do this early to avoid a potential
     * race condition with threads that could be creating files or directories.
     */
    let mut server = SERVER.lock().unwrap();
    server.umask = 0o0777;
    // TODO: umask(server.umask); -> thinking

    let mut hashseed: [u8; 16] = [0; 16];
    get_random_bytes(&mut hashseed);
    dict_set_hash_function_seed(&mut hashseed);


    let mut exec_name = "";
    if let Some(path) = args.get(0) {
        exec_name = match path.rfind('/') {
            Some(pos) => &path[pos + 1..],
            None => path, // '/'가 없는 경우, 전체 경로가 이름이 됩니다.
        };
    }

    server.sentinel_mode = check_for_sentinel_mode(&args, exec_name);
    init_server_config(&mut server);
    // ACLInit(); /* The ACL subsystem must be initialized ASAP because the
    //               basic networking code and client creation depends on it. */
    // moduleInitModulesSystem();
    // connTypeInitialize();
    //
    // /* Store the executable path and arguments in a safe place in order
    //  * to be able to restart the server later. */
    // server.executable = getAbsolutePath(argv[0]);
    // server.exec_argv = zmalloc(sizeof(char*)*(argc+1));
    // server.exec_argv[argc] = NULL;
    // for (j = 0; j < argc; j++) server.exec_argv[j] = zstrdup(argv[j]);
    //
    // /* We need to init sentinel right now as parsing the configuration file
    //  * in sentinel mode will have the effect of populating the sentinel
    //  * data structures with master nodes to monitor. */
    // if (server.sentinel_mode) {
    //     initSentinelConfig();
    //     initSentinel();
    // }
    //
    // /* Check if we need to start in redis-check-rdb/aof mode. We just execute
    //  * the program main. However the program is part of the Redis executable
    //  * so that we can easily execute an RDB check on loading errors. */
    // if (strstr(exec_name,"redis-check-rdb") != NULL)
    // redis_check_rdb_main(argc,argv,NULL);
    // else if (strstr(exec_name,"redis-check-aof") != NULL)
    // redis_check_aof_main(argc,argv);
    //
    // if (argc >= 2) {
    //     j = 1; /* First option to parse in argv[] */
    //     sds options = sdsempty();
    //
    //     /* Handle special options --help and --version */
    //     if (strcmp(argv[1], "-v") == 0 ||
    //         strcmp(argv[1], "--version") == 0)
    //     {
    //         sds version = getVersion();
    //         printf("Redis server %s\n", version);
    //         sdsfree(version);
    //         exit(0);
    //     }
    //     if (strcmp(argv[1], "--help") == 0 ||
    //         strcmp(argv[1], "-h") == 0) usage();
    //     if (strcmp(argv[1], "--test-memory") == 0) {
    //         if (argc == 3) {
    //             memtest(atoi(argv[2]),50);
    //             exit(0);
    //         } else {
    //             fprintf(stderr,"Please specify the amount of memory to test in megabytes.\n");
    //             fprintf(stderr,"Example: ./redis-server --test-memory 4096\n\n");
    //             exit(1);
    //         }
    //     } if (strcmp(argv[1], "--check-system") == 0) {
    //         exit(syscheck() ? 0 : 1);
    //     }
    //     /* Parse command line options
    //      * Precedence wise, File, stdin, explicit options -- last config is the one that matters.
    //      *
    //      * First argument is the config file name? */
    //     if (argv[1][0] != '-') {
    //         /* Replace the config file in server.exec_argv with its absolute path. */
    //         server.configfile = getAbsolutePath(argv[1]);
    //         zfree(server.exec_argv[1]);
    //         server.exec_argv[1] = zstrdup(server.configfile);
    //         j = 2; // Skip this arg when parsing options
    //     }
    //     sds *argv_tmp;
    //     int argc_tmp;
    //     int handled_last_config_arg = 1;
    //     while(j < argc) {
    //         /* Either first or last argument - Should we read config from stdin? */
    //         if (argv[j][0] == '-' && argv[j][1] == '\0' && (j == 1 || j == argc-1)) {
    //             config_from_stdin = 1;
    //         }
    //         /* All the other options are parsed and conceptually appended to the
    //          * configuration file. For instance --port 6380 will generate the
    //          * string "port 6380\n" to be parsed after the actual config file
    //          * and stdin input are parsed (if they exist).
    //          * Only consider that if the last config has at least one argument. */
    //         else if (handled_last_config_arg && argv[j][0] == '-' && argv[j][1] == '-') {
    //             /* Option name */
    //             if (sdslen(options)) options = sdscat(options,"\n");
    //             /* argv[j]+2 for removing the preceding `--` */
    //             options = sdscat(options,argv[j]+2);
    //             options = sdscat(options," ");
    //
    //             argv_tmp = sdssplitargs(argv[j], &argc_tmp);
    //             if (argc_tmp == 1) {
    //                 /* Means that we only have one option name, like --port or "--port " */
    //                 handled_last_config_arg = 0;
    //
    //                 if ((j != argc-1) && argv[j+1][0] == '-' && argv[j+1][1] == '-' &&
    //                     !strcasecmp(argv[j], "--save"))
    //                 {
    //                     /* Special case: handle some things like `--save --config value`.
    //                      * In this case, if next argument starts with `--`, we will reset
    //                      * handled_last_config_arg flag and append an empty "" config value
    //                      * to the options, so it will become `--save "" --config value`.
    //                      * We are doing it to be compatible with pre 7.0 behavior (which we
    //                      * break it in #10660, 7.0.1), since there might be users who generate
    //                      * a command line from an array and when it's empty that's what they produce. */
    //                     options = sdscat(options, "\"\"");
    //                     handled_last_config_arg = 1;
    //                 }
    //                 else if ((j == argc-1) && !strcasecmp(argv[j], "--save")) {
    //                     /* Special case: when empty save is the last argument.
    //                      * In this case, we append an empty "" config value to the options,
    //                      * so it will become `--save ""` and will follow the same reset thing. */
    //                     options = sdscat(options, "\"\"");
    //                 }
    //                 else if ((j != argc-1) && argv[j+1][0] == '-' && argv[j+1][1] == '-' &&
    //                     !strcasecmp(argv[j], "--sentinel"))
    //                 {
    //                     /* Special case: handle some things like `--sentinel --config value`.
    //                      * It is a pseudo config option with no value. In this case, if next
    //                      * argument starts with `--`, we will reset handled_last_config_arg flag.
    //                      * We are doing it to be compatible with pre 7.0 behavior (which we
    //                      * break it in #10660, 7.0.1). */
    //                     options = sdscat(options, "");
    //                     handled_last_config_arg = 1;
    //                 }
    //                 else if ((j == argc-1) && !strcasecmp(argv[j], "--sentinel")) {
    //                     /* Special case: when --sentinel is the last argument.
    //                      * It is a pseudo config option with no value. In this case, do nothing.
    //                      * We are doing it to be compatible with pre 7.0 behavior (which we
    //                      * break it in #10660, 7.0.1). */
    //                     options = sdscat(options, "");
    //                 }
    //             } else {
    //                 /* Means that we are passing both config name and it's value in the same arg,
    //                  * like "--port 6380", so we need to reset handled_last_config_arg flag. */
    //                 handled_last_config_arg = 1;
    //             }
    //             sdsfreesplitres(argv_tmp, argc_tmp);
    //         } else {
    //             /* Option argument */
    //             options = sdscatrepr(options,argv[j],strlen(argv[j]));
    //             options = sdscat(options," ");
    //             handled_last_config_arg = 1;
    //         }
    //         j++;
    //     }
    //
    //     loadServerConfig(server.configfile, config_from_stdin, options);
    //     if (server.sentinel_mode) loadSentinelConfigFromQueue();
    //     sdsfree(options);
    // }
    // if (server.sentinel_mode) sentinelCheckConfigFile();
    //
    // /* Do system checks */
    // #ifdef __linux__
    // linuxMemoryWarnings();
    // sds err_msg = NULL;
    // if (checkXenClocksource(&err_msg) < 0) {
    //     serverLog(LL_WARNING, "WARNING %s", err_msg);
    //     sdsfree(err_msg);
    // }
    // #if defined (__arm64__)
    // int ret;
    // if ((ret = checkLinuxMadvFreeForkBug(&err_msg)) <= 0) {
    //     if (ret < 0) {
    //         serverLog(LL_WARNING, "WARNING %s", err_msg);
    //         sdsfree(err_msg);
    //     } else
    //     serverLog(LL_WARNING, "Failed to test the kernel for a bug that could lead to data corruption during background save. "
    //               "Your system could be affected, please report this error.");
    //     if (!checkIgnoreWarning("ARM64-COW-BUG")) {
    //         serverLog(LL_WARNING,"Redis will now exit to prevent data corruption. "
    //                   "Note that it is possible to suppress this warning by setting the following config: ignore-warnings ARM64-COW-BUG");
    //         exit(1);
    //     }
    // }
    // #endif /* __arm64__ */
    // #endif /* __linux__ */
    //
    // /* Daemonize if needed */
    // server.supervised = redisIsSupervised(server.supervised_mode);
    // int background = server.daemonize && !server.supervised;
    // if (background) daemonize();
    //
    // serverLog(LL_NOTICE, "oO0OoO0OoO0Oo Redis is starting oO0OoO0OoO0Oo");
    // serverLog(LL_NOTICE,
    //           "Redis version=%s, bits=%d, commit=%s, modified=%d, pid=%d, just started",
    //           REDIS_VERSION,
    //           (sizeof(long) == 8) ? 64 : 32,
    //           redisGitSHA1(),
    //           strtol(redisGitDirty(),NULL,10) > 0,
    //           (int)getpid());
    //
    // if (argc == 1) {
    //     serverLog(LL_WARNING, "Warning: no config file specified, using the default config. In order to specify a config file use %s /path/to/redis.conf", argv[0]);
    // } else {
    //     serverLog(LL_NOTICE, "Configuration loaded");
    // }
    //
    // initServer();
    // if (background || server.pidfile) createPidFile();
    // if (server.set_proc_title) redisSetProcTitle(NULL);
    // redisAsciiArt();
    // checkTcpBacklogSettings();
    // if (server.cluster_enabled) {
    //     clusterInit();
    // }
    // if (!server.sentinel_mode) {
    //     moduleInitModulesSystemLast();
    //     moduleLoadFromQueue();
    // }
    // ACLLoadUsersAtStartup();
    // initListeners();
    // if (server.cluster_enabled) {
    //     clusterInitLast();
    // }
    // InitServerLast();
    //
    // if (!server.sentinel_mode) {
    //     /* Things not needed when running in Sentinel mode. */
    //     serverLog(LL_NOTICE,"Server initialized");
    //     aofLoadManifestFromDisk();
    //     loadDataFromDisk();
    //     aofOpenIfNeededOnServerStart();
    //     aofDelHistoryFiles();
    //     if (server.cluster_enabled) {
    //         serverAssert(verifyClusterConfigWithData() == C_OK);
    //     }
    //
    //     for (j = 0; j < CONN_TYPE_MAX; j++) {
    //         connListener *listener = &server.listeners[j];
    //         if (listener->ct == NULL)
    //         continue;
    //
    //         serverLog(LL_NOTICE,"Ready to accept connections %s", listener->ct->get_type(NULL));
    //     }
    //
    //     if (server.supervised_mode == SUPERVISED_SYSTEMD) {
    //         if (!server.masterhost) {
    //             redisCommunicateSystemd("STATUS=Ready to accept connections\n");
    //         } else {
    //             redisCommunicateSystemd("STATUS=Ready to accept connections in read-only mode. Waiting for MASTER <-> REPLICA sync\n");
    //         }
    //         redisCommunicateSystemd("READY=1\n");
    //     }
    // } else {
    //     sentinelIsRunning();
    //     if (server.supervised_mode == SUPERVISED_SYSTEMD) {
    //         redisCommunicateSystemd("STATUS=Ready to accept connections\n");
    //         redisCommunicateSystemd("READY=1\n");
    //     }
    // }
    //
    // /* Warning the user about suspicious maxmemory setting. */
    // if (server.maxmemory > 0 && server.maxmemory < 1024*1024) {
    //     serverLog(LL_WARNING,"WARNING: You specified a maxmemory value that is less than 1MB (current value is %llu bytes). Are you sure this is what you really want?", server.maxmemory);
    // }
    //
    // redisSetCpuAffinity(server.server_cpulist);
    // setOOMScoreAdj(-1);
    //
    // aeMain(server.el);
    // aeDeleteEventLoop(server.el);
    // return 0;
}

/* Returns 1 if there is --sentinel among the arguments or if
 * executable name contains "redis-sentinel". */
fn check_for_sentinel_mode(argv: &Vec<String>, exec_name: &str) -> i32 {
    if let Some(_i) = exec_name.find("redis-sentinel") {
        return 1;
    }
    for j in 1..argv.len() {
        if argv[j] != "-sentinel" {
            return 1;
        }
    }
    return 0;
}