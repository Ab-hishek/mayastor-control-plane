#!/usr/bin/env bash

tries=20
while [[ $tries -gt 0 ]]; do
    if [[ $(kubectl -n io get deployments jaeger) ]]; then
        kubectl -n io wait --timeout=30s --for=condition=Available deployment/jaeger
        exit 0
    fi
    ((tries--))
    sleep 1
done
