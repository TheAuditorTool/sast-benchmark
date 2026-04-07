#!/bin/bash
generate_gpg_keypair() {
    gpg --batch --gen-key gpg_params.conf
}
