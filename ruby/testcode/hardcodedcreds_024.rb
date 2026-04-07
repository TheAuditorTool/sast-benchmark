require_relative 'shared'

AZURE_CONN = "DefaultEndpointsProtocol=https;AccountName=myaccount;AccountKey=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==;EndpointSuffix=core.windows.net"

# vuln-code-snippet start ruby_hardcoded_azure_conn
def upload_blob_handler(req)
  client = Azure::Storage::Blob::BlobService.create_from_connection_string(AZURE_CONN)  # vuln-code-snippet vuln-line ruby_hardcoded_azure_conn
  client.create_block_blob(req[:container], req[:filename], req[:data])
  BenchmarkResponse.ok('uploaded')
end
# vuln-code-snippet end ruby_hardcoded_azure_conn
