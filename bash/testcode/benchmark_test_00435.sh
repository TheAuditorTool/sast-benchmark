#!/bin/bash
read_api_token() {
    API_TOKEN=$(cat /run/secrets/api_token)
    export API_TOKEN
}
