#!/usr/bin/env bash

# export PACKAGE_VERSION=$(awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml)
# export PACKAGE_NAME=$(awk -F ' = ' '$1 ~ /name/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml)
# export REGISTRY_URL=$(awk -F ' = ' '$1 ~ /registry/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml)
#
# docker buildx build --platform linux/amd64 -t "${REGISTRY_URL}/${PACKAGE_NAME}:${PACKAGE_VERSION}" --push .
# docker buildx build --platform linux/amd64 -t "${REGISTRY_URL}/${PACKAGE_NAME}:latest" --push .
# docker buildx build --platform linux/amd64 -t "${REGISTRY_URL}/${PACKAGE_NAME}:stable" --push .

PACKAGE_VERSION=$(awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml)
echo "$PACKAGE_VERSION"
