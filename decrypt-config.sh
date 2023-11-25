#!/bin/sh

if [ -z "$1" ]; then
    echo "Pass environment of the configuration file (the config.<env>.toml.gpg)"
    exit 1
fi
if [ -z "${CONFIGURATION_PASSWORD}" ]; then
    echo "The CONFIGURATION_PASSWORD variable is not set - refusing to run."
    exit 1
fi

gpg --quiet --batch --yes --decrypt --passphrase="$CONFIGURATION_PASSWORD" --output config.$1.toml config.$1.toml.gpg
