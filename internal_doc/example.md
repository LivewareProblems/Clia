# Example

This is the control script of erlang/elixir releases, first the original in
posix shell [as found in
elixir](https://github.com/elixir-lang/elixir/blob/main/lib/mix/lib/mix/tasks/release.init.ex#L81-L269)
then rewritten in a possible syntax for Clia.

```shell
    #!/bin/sh
    set -e

    SELF=$(readlink "$0" || true)
    if [ -z "$SELF" ]; then SELF="$0"; fi
    RELEASE_ROOT="$(CDPath='' cd "$(dirname "$SELF")/.." && pwd -P)"
    export RELEASE_ROOT
    RELEASE_NAME="${RELEASE_NAME:-"test_app"}"
    export RELEASE_NAME
    RELEASE_VSN="${RELEASE_VSN:-"$(cut -d' ' -f2 "$RELEASE_ROOT/releases/start_erl.data")"}"
    export RELEASE_VSN
    RELEASE_COMMAND="$1"
    export RELEASE_COMMAND
    RELEASE_PROG="${RELEASE_PROG:-"$(echo "$0" | sed 's/.*\///')"}"
    export RELEASE_PROG

    REL_VSN_DIR="$RELEASE_ROOT/releases/$RELEASE_VSN"
    . "$REL_VSN_DIR/env.sh"

    RELEASE_COOKIE="${RELEASE_COOKIE:-"$(cat "$RELEASE_ROOT/releases/COOKIE")"}"
    export RELEASE_COOKIE
    RELEASE_MODE="${RELEASE_MODE:-"embedded"}"
    export RELEASE_MODE
    RELEASE_NODE="${RELEASE_NODE:-"$RELEASE_NAME"}"
    export RELEASE_NODE
    RELEASE_TMP="${RELEASE_TMP:-"$RELEASE_ROOT/tmp"}"
    export RELEASE_TMP
    RELEASE_VM_ARGS="${RELEASE_VM_ARGS:-"$REL_VSN_DIR/vm.args"}"
    export RELEASE_VM_ARGS
    RELEASE_REMOTE_VM_ARGS="${RELEASE_REMOTE_VM_ARGS:-"$REL_VSN_DIR/remote.vm.args"}"
    export RELEASE_REMOTE_VM_ARGS
    RELEASE_DISTRIBUTION="${RELEASE_DISTRIBUTION:-"sname"}"
    export RELEASE_DISTRIBUTION
    RELEASE_BOOT_SCRIPT="${RELEASE_BOOT_SCRIPT:-"start"}"
    export RELEASE_BOOT_SCRIPT
    RELEASE_BOOT_SCRIPT_CLEAN="${RELEASE_BOOT_SCRIPT_CLEAN:-"start_clean"}"
    export RELEASE_BOOT_SCRIPT_CLEAN

    rand () {
      dd count=1 bs=2 if=/dev/urandom 2> /dev/null | od -x | awk 'NR==1{print $2}'
    }

    release_distribution () {
      case $RELEASE_DISTRIBUTION in
        none)
          ;;

        name | sname)
          echo "--$RELEASE_DISTRIBUTION $1"
          ;;

        *)
          echo "ERROR: Expected sname, name, or none in RELEASE_DISTRIBUTION, got: $RELEASE_DISTRIBUTION" >&2
          exit 1
          ;;
      esac
    }

    rpc () {
      exec "$REL_VSN_DIR/elixir" \
           --hidden --cookie "$RELEASE_COOKIE" \
           $(release_distribution "rpc-$(rand)-$RELEASE_NODE") \
           --boot "$REL_VSN_DIR/$RELEASE_BOOT_SCRIPT_CLEAN" \
           --boot-var RELEASE_LIB "$RELEASE_ROOT/lib" \
           --vm-args "$RELEASE_REMOTE_VM_ARGS" \
           --rpc-eval "$RELEASE_NODE" "$1"
    }

    start () {
      export_release_sys_config
      REL_EXEC="$1"
      shift
      exec "$REL_VSN_DIR/$REL_EXEC" \
           --cookie "$RELEASE_COOKIE" \
           $(release_distribution "$RELEASE_NODE") \
           --erl "-mode $RELEASE_MODE" \
           --erl-config "$RELEASE_SYS_CONFIG" \
           --boot "$REL_VSN_DIR/$RELEASE_BOOT_SCRIPT" \
           --boot-var RELEASE_LIB "$RELEASE_ROOT/lib" \
           --vm-args "$RELEASE_VM_ARGS" "$@"
    }

    export_release_sys_config () {
      DEFAULT_SYS_CONFIG="${RELEASE_SYS_CONFIG:-"$REL_VSN_DIR/sys"}"

      if grep -q "RUNTIME_CONFIG=true" "$DEFAULT_SYS_CONFIG.config"; then
        RELEASE_SYS_CONFIG="$RELEASE_TMP/$RELEASE_NAME-$RELEASE_VSN-$(date +%Y%m%d%H%M%S)-$(rand).runtime"

        (mkdir -p "$RELEASE_TMP" && cat "$DEFAULT_SYS_CONFIG.config" >"$RELEASE_SYS_CONFIG.config") || (
          echo "ERROR: Cannot start release because it could not write $RELEASE_SYS_CONFIG.config" >&2
          exit 1
        )
      else
        RELEASE_SYS_CONFIG="$DEFAULT_SYS_CONFIG"
      fi

      export RELEASE_SYS_CONFIG
    }

    case $1 in
      start)
        start "elixir" --no-halt
        ;;

      start_iex)
        start "iex" --werl
        ;;

      daemon)
        start "elixir" --no-halt --pipe-to "${RELEASE_TMP}/pipe" "${RELEASE_TMP}/log"
        ;;

      daemon_iex)
        start "iex" --pipe-to "${RELEASE_TMP}/pipe" "${RELEASE_TMP}/log"
        ;;

      eval)
        if [ -z "$2" ]; then
          echo "ERROR: EVAL expects an expression as argument" >&2
          exit 1
        fi
        script="$2"
        shift 2
        export_release_sys_config
        exec "$REL_VSN_DIR/elixir" \
           --cookie "$RELEASE_COOKIE" \
           --erl-config "$RELEASE_SYS_CONFIG" \
           --boot "$REL_VSN_DIR/$RELEASE_BOOT_SCRIPT_CLEAN" \
           --boot-var RELEASE_LIB "$RELEASE_ROOT/lib" \
           --vm-args "$RELEASE_VM_ARGS" --eval "$script" -- "$@"
        ;;

      remote)
        exec "$REL_VSN_DIR/iex" \
             --werl --hidden --cookie "$RELEASE_COOKIE" \
             $(release_distribution "rem-$(rand)-$RELEASE_NODE") \
             --boot "$REL_VSN_DIR/$RELEASE_BOOT_SCRIPT_CLEAN" \
             --boot-var RELEASE_LIB "$RELEASE_ROOT/lib" \
             --vm-args "$RELEASE_REMOTE_VM_ARGS" \
             --remsh "$RELEASE_NODE"
        ;;

      rpc)
        if [ -z "$2" ]; then
          echo "ERROR: RPC expects an expression as argument" >&2
          exit 1
        fi
        rpc "$2"
        ;;

      restart|stop)
        rpc "System.$1()"
        ;;

      pid)
        rpc "IO.puts System.pid()"
        ;;

      version)
        echo "$RELEASE_NAME $RELEASE_VSN"
        ;;

      *)
        echo "Usage: $(basename "$0") COMMAND [ARGS]

    The known commands are:

        start          Starts the system
        start_iex      Starts the system with IEx attached
        daemon         Starts the system as a daemon
        daemon_iex     Starts the system as a daemon with IEx attached
        eval \"EXPR\"    Executes the given expression on a new, non-booted system
        rpc \"EXPR\"     Executes the given expression remotely on the running system
        remote         Connects to the running system via a remote shell
        restart        Restarts the running system via a remote command
        stop           Stops the running system via a remote command
        pid            Prints the operating system PID of the running system via a remote command
        version        Prints the release name and version to be booted
    " >&2

        if [ -n "$1" ]; then
          echo "ERROR: Unknown command $1" >&2
          exit 1
        fi
        ;;
    esac
```

## Clia Version

And here is a possible clia version. Note that this is a pure invention. There
are no compiler, nor language specification, yet. And this could change at any
point. But this is what the syntax could look like. The code is heavily
commented in order to demonstrate the features of Clia that are on display.
We are not particularly attached to some of these syntaxes, this is just a wip.

```elixir
#!/bin/clia

# This tells Clia that we are writing a script. This allows Clia
# to load a specific prelude but also to have some default callback
# added. These Behaviour could be 3rd party, but in this case
# this is the behaviour for a script that ship with Clia
# It provides:
# 1. a lifecycle for the entry points of the program, and their type spec
# 2. also tells clia to auto import all the System functions so we can call
#   them without their fully qualified name
# 3. Allows to define commands, args and flag parsing pragmas
@behaviour Script

# We register commands, with the @command shortcut imported by the Script behaviour
# The @command function take as input the name of the commmand and a function capture that point to the function that will handle that command
# These functions need to implement the Script.Command behaviour callback contract
# The `&` operator is used to pass a function reference.
@command "start", call: &start,
                  args: [%{name: "ELIXIR_FLAGS", type: String, required: false, default: ""}],
                  doc: "Starts the system"

@command "start_iex", call: &start_iex,
                  args: [%{name: "IEX_FLAGS", type: String, required: false, default: ""}],
                  doc: "Starts the system with IEx attached"

@command "daemon", call: &daemon,
                   args: [%{name: "ELIXIR_FLAGS", type: String, required: false, default: ""}],
                   doc: "Starts the system as a daemon"

@command "daemon_iex", call: &daemon_iex,
                       args: [%{name: "IEX_FLAGS", type: String, required: false, default: ""}],
                       doc: "Starts the system as a daemon with IEx attached"

@command "eval", call: &eval,
                 args: [%{name: "EXPR", type: String, required: true},
                        %{name: "ELIXIR_FLAGS", type: String, required: false, default: ""}],
                 doc: "Executes the given expression on a new, non-booted system"

@command "rpc", call: &rpc,
                args: [%{name: "EXPR", type: String, required: true}],
                doc: "Executes the given expression remotely on the running system"

@command "remote", call: &remote,
                   doc: "Connects to the running system via a remote shell"

@command "restart", call: &restart,
                    doc: "Restarts the running system via a remote command"

@command "stop", call: &stop,
                 doc: "Stops the running system via a remote command"

@command "pid", call: &pid,
                doc: "Prints the operating system PID of the running system via a remote command"

@command "version", call: &version,
                    doc: "Prints the release name and version to be booted"

# impl tell Clia that this function implement the init callback of the Script behaviour.
# This automatically apply the type spec defined in Script.Init and place it in the lifecycle
# In this case it will be run before the commands but after parsing of the command, args and flags passed in the ctx structure
@impl Script.Init
def init(ctx) do

  # ~SIGIL is a tagged template. Path allo safe interpolation for clia terms in curly brackets into a path.
  # The output of the interpolation would be escaped, or if it could not be interpolated safely, it would
  # fail to be type checked or in scripting mode woudld safely panic.
  # get_own_path is also capability protected and will demand for user consent.
  # These can be saved to a file linked to this script, avoiding having to answer all capabilities for every run
  release_root = System.expand(~Path"{get_own_path()}/..")
  release_vsn = extract_release_vsn(release_root)
  release_vsn_dir = ~Path"{release_root}/releases/{release_vsn}"

  # The script behaviour import all of System module function, as it expect you will use them a lot
  # This means we can call drop the System in System.get_env()
  # Note that the environment is also capabilities protected.
  env = 
    get_env()
    |> put_new(:release_root, release_root)
    # |> is the pipeline operator, it push the return value of the left operand in the first argument of the right operand
    |> put_new(:release_name, "test_app")
    # put_new add a key/value if the key is not already in the map and the value is not "empty"
    |> put_new(:release_vsn, release_root)
    |> put_new(:release_command, ctx.command)
    |> put_new_lazy(:release_prog, extract_release_prog(ctx.self))
    |> put_new(:rel_vsn_dir, rel_vsn_dir)
    |> merge(Shell.parse_export(~Path"{rel_vsn_dir}/env.sh"))
    |> put_new( :release_cookie, extract_cookie(release_root))
    |> put_new( :release_mode, "embedded")

  # Clia allow shadowing a previously binding variable name.
  # note that this is not mutability. We are shadowing and hiding the previous binding only in the rest the lexical block
  # If env was closed over before this, it will still have the value from the binding that was closed over.
  env =
    env
    |> put_new( :release_node, env.release_name))
    |> put_new( :release_tmp, ~Path"{release_root}/tmp")
    |> put_new( :release_vm_args, ~Path"{rel_vsn_dir}/vm.args")
    |> put_new( :release_remote_vm_args, ~Path"{rel_vsn_dir}/remote.vm.args")
    |> put_new( :release_distribution, "sname")
    |> put_new( :release_boot_script, "start")
    |> put_new( :release_boot_script_clean, "start_clean")

  System.put_env(env)
  # Clia nearly only has expressions. As such, we do not use the `return` keyword
  # functions simply return the value of their last expression
  ctx
end

# Unused argument can be ignored with an underscore at the start of their binding
@impl Script.Command
def start(_ctx, [flags]) do
  do_start("elixir", String.join(["--no-halt", flags], " "))
end

@impl Script.Command
def start_iex(_ctx, [flags]) do
  do_start("iex", String.join(["--werl", flags], " "))
end

# Unused argument can be ignored with an underscore too if we do not care about naming them
@impl Script.Command
def daemon(_, [flags]) do
  release_temp = get_env().release_tmp

  # We can use `sigil_path` instead of `~Path` as the later is syntax sugar for the former
  do_start (
    "elixir",
    String.join(
      [
        "--no-halt --pipe-to",
        "#{sigil_Path("{release_tmp}/pipe")}".
        "#{sigil_Path("{release_tmp}/log")}",
        flags
      ],
      " "
    )
  )
end

@impl Script.Command
def daemon_iex(_ctx, [flags]) do

  release_temp = get_env().release_tmp

  do_start(
    "iex",
    String.join(
      [
        "--no-halt --pipe-to",
        "#{sigil_Path("{release_tmp}/pipe")}".
        "#{sigil_Path("{release_tmp}/log")}",
        flags
      ],
      " "
    )
  )
end


# We run a shell command here. This will ask for a capability to the user.
# We could also register a global capability, asking it to the user at the start
# instead of when we start the program
# We also are not handling any of the effects Shell could generate, so the program
# will panic at runtime. We are considering enforcing these at compile time
# If that is the case, we will probably allow a catchall for Scripts that will just panic
def do_start(command, flags) do
  export_release_sys_config()

  env = get_env()

  # Here we see a sigil inside a sigil. This is possible ...
  # But we need to use a different set of delimitors.
  # Luckily, sigils accepter different delimitors, like square brackets here
  command = ~Shell"""
      "{~Path[{env.rel_vsn_dir}/{command}]}" \
          --cookie "{env.release_cookie}" \
          {release_distribution(env.release_node)} \
          --erl "-mode {env.release_mode}" \
          --erl-config "{env.release_sys_config}" \
          --boot "{~Path[{env.rel_vsn_dir}/{env.release_boot_script}]" \
          --boot-var RELEASE_LIB "{~Path[{env.release_root}/lib]" \
          --vm-args "{env.release_vm_args}" "{flags}"
    """
    System.exec(command)
end

@impl Script.Command
def eval(_ctx, [expression, flags]) do
  export_release_sys_config()

  env = get_env()

  command = ~Shell"""
    "{~Path[{env.rel_vsn_dir}/elixir]}" \
        --cookie "{env.release_cookie}" \
        --erl-config "{env.release_sys_config}" \
        --boot "{~Path[{env.rel_vsn_dir}/{env.release_boot_script_clean}]" \
        --boot-var RELEASE_LIB "{~Path[{env.release_root}/lib]" \
        --vm-args "{env.release_vm_args}"
        --eval "{expression}" -- "{flags}"
  """

  System.exec(command)
end

@impl Script.Command
def remote(_ctx) do
  export_release_sys_config()

  env = get_env()

  command = ~Shell"""
      "{~Path[{env.rel_vsn_dir}/iex]}" \
          --werl --hidden --cookie "{env.release_cookie}" \
          {release_distribution(rand() <> "-" <> env.release_node)} \
          --boot "{~Path[{env.rel_vsn_dir}/{env.release_boot_script_clean}]" \
          --boot-var RELEASE_LIB "{~Path[{env.release_root}/lib]" \
          --vm-args "{env.release_remote_vm_args}" \
          --remsh "{env.release_node}"
    """
    System.exec(command)
end

@impl Script.Command
def rpc(_ctx, [expression]) do
  do_rpc(expression)
end

@impl Script.Command
def restart(_ctx) do
  do_rpc("System.restart()")
end

@impl Script.Command
def stop(_ctx) do
  do_rpc("System.stop()")
end

@impl Script.Command
def pid(_ctx) do
  do_rpc("IO.puts System.pid()")
end

def do_rpc(expression) do
  env = get_env()

  command = ~Shell"""
    "{~Path[{env.rel_vsn_dir}/elixir]}" \
        --hidden --cookie "{env.release_cookie}" \
        {release_distribution(rand() <> "-" <> env.release_node)} \
        --boot "{~Path[{env.rel_vsn_dir}/{env.release_boot_script_clean}]" \
        --boot-var RELEASE_LIB "{~Path[{env.release_root}/lib]" \
        --vm-args "{env.release_remote_vm_args}"
        --rpc-eval "{env.release_node}" "{expression}"
  """

  System.exec(command)
end

@impl Script.Command
def version(_ctx) do
  env = get_env()

  Console.puts("#{env.release_name} #{env.release_vsn}")
end

def extract_release_vsn(release_root) do
  cmd(~Shell(cut -d' ' -f2 "{~Path[{release_root}/releases/start_erl.data]"))
end

# Effects of functions need to be handled at the caller level.
# if they are not handled at the call site, they are considered part of the Effect
# set of the function (like a capability) and as such need to be handled
# higher in the lexical call stack.
# To handle an effect, we use the `with` keyword and pattern match
# The return type should be the one of the function that generated the effect
def extract_release_prog(self) do
  # This regex in sed is probably wrong in term of escapes, but i do not want to think about these semantics for now.
  cmd(~Shell(echo "{self}" | sed 's/.*\///'))
with
  Error(_) -> ""
end

def extract_cookie(release_root) do
  File.read(~Path"{release_root}/releases/COOKIE")
  |> String.parse()
with
  # `panic` behave like a Rust panic, it is basically an emergency exit.
  PosixError(msg) -> panic("extracting cookies had a problem with error #{msg}")
end

def rand() do
  Random.bytes(2)
  |> Integer.to_string(16)
end

def export_release_sys_config() do
  env = get_env()
  default_sys_config= Map.get("RELEASE_SYS_CONFIG",~Path"{env.rel_vsn_dir}/sys")

  # Effects can be handled at the call site directly by using a do/with/end block
  # Effects can resume, returning the argument of the resumption at the call site
  # The type of the effect handler need to match the type of the effect return
  # In the case of PosixError, it is actually a polymorphic type under the hood
  runtime_config? = 
    do
      File.read(~Path"{default_sys_config}.config")
      |> String.contains?("RUNTIME_CONFIG=true")
    with
      PosixError(_type) -> resume("")
    end

  release_sys_config = 
    if runtime_config? do
      time = System.cmd(~Shell"date", ~Shell"+%Y%m%d%H%M%S")
      release_sys_config = 
        ~Path"{env.release_tmp}/{env.realease_name}-{env.release_vsn}-{time}-{rand()}.runtime"

      do
        File.mkdir_p(env.release_tmp)
        File.copy(~Path"{default_sys_config.config}",~Path"{release_sys_config}.config")
      with
        PosixError(_type) ->
          panic("Cannot start release because it could not write #{release_sys_config}.config")
      end
      release_sys_config
    else
      default_sys_config
    end

  System.put_env(env, :release_sys_config, release_sys_config)
end

def release_distribution(name) do
  case System.get_env(:release_distribution) do
    "none" -> ""
    type in ["name", "sname"] -> ~Shell"--{type} {name}"
    wrong -> panic("ERROR: Expected sname, name, or none in RELEASE_DISTRIBUTION, got: #{wrong}")
  end
end

```
