"""Cloud metadata proxy -- IDENTICAL between vuln/ and safe/.

Simulates a cloud environment where the instance metadata service
at 169.254.169.254 returns IAM credentials, instance identity,
and user-data scripts. This is the target of the SSRF chain.
"""

# This file represents the cloud metadata service that is reachable
# from the application's network. The metadata endpoint returns
# sensitive data (IAM role credentials, instance identity tokens)
# to any process on the instance that can make HTTP requests to it.
#
# In a real cloud environment, this is http://169.254.169.254/latest/meta-data/
# An SSRF vulnerability in gateway.py allows an external attacker to
# reach this endpoint through the application server.
#
# Metadata endpoints exposed:
#   /latest/meta-data/iam/security-credentials/<role-name>
#     -> Returns temporary AWS credentials (AccessKeyId, SecretAccessKey, Token)
#   /latest/meta-data/instance-id
#     -> Returns the EC2 instance ID
#   /latest/user-data
#     -> Returns the instance user-data script (may contain secrets)

METADATA_RESPONSES = {
    "/latest/meta-data/iam/security-credentials/app-role": {
        "AccessKeyId": "ASIAXXXXXXXXXEXAMPLE",
        "SecretAccessKey": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
        "Token": "FwoGZXIvYXdzEBYaDH...",
        "Expiration": "2026-04-01T12:00:00Z",
    },
    "/latest/meta-data/instance-id": "i-0abcd1234efgh5678",
    "/latest/user-data": "#!/bin/bash\nexport DB_PASSWORD='production-secret-2026'\n",
}
