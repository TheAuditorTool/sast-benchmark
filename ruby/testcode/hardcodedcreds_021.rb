require_relative 'shared'

GCP_CREDENTIALS = {
  type: 'service_account',
  private_key: "-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEA0Z3VS5JJcds3xHn/ygWep4PAtEsHAAAAAAAAAAAAAACfj5t0\n-----END RSA PRIVATE KEY-----\n",
  client_email: 'app@project.iam.gserviceaccount.com'
}.freeze

# vuln-code-snippet start ruby_hardcoded_gcp_key
def gcp_auth_handler(req)
  creds = Google::Auth::ServiceAccountCredentials.make_creds(json_key_io: StringIO.new(GCP_CREDENTIALS.to_json))  # vuln-code-snippet vuln-line ruby_hardcoded_gcp_key
  creds.fetch_access_token!
  BenchmarkResponse.json({ token: creds.access_token })
end
# vuln-code-snippet end ruby_hardcoded_gcp_key
