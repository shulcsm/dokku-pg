export DOKKU_ROOT="/home/dokku"
export DOKKU_DISTRO="ubuntu"

export DOKKU_IMAGE="gliderlabs/herokuish"
export DOKKU_LIB_ROOT="/var/lib/dokku"

export PLUGIN_PATH="$DOKKU_LIB_ROOT/plugins"
export PLUGIN_AVAILABLE_PATH="$PLUGIN_PATH/available"
export PLUGIN_ENABLED_PATH="$PLUGIN_PATH/enabled"
export PLUGIN_CORE_PATH="$DOKKU_LIB_ROOT/core-plugins"
export PLUGIN_CORE_AVAILABLE_PATH="$PLUGIN_CORE_PATH/available"
export PLUGIN_CORE_ENABLED_PATH="$PLUGIN_CORE_PATH/enabled"

export DOKKU_API_VERSION=1
export DOKKU_NOT_IMPLEMENTED_EXIT=10
export DOKKU_VALID_EXIT=0

export DOKKU_LOGS_DIR="/var/log/dokku"
export DOKKU_EVENTS_LOGFILE="$DOKKU_LOGS_DIR/events.log"

export DOKKU_CONTAINER_LABEL=dokku
export DOKKU_GLOBAL_RUN_ARGS="--label=$DOKKU_CONTAINER_LABEL"
