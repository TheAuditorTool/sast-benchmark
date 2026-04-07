require_relative 'shared'

AZURE_CONN = "DefaultEndpointsProtocol=https;AccountName=myaccount;AccountKey=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==;EndpointSuffix=core.windows.net"

def upload_blob_handler(req)
  client = Azure::Storage::Blob::BlobService.create_from_connection_string(AZURE_CONN)
  client.create_block_blob(req[:container], req[:filename], req[:data])
  BenchmarkResponse.ok('uploaded')
end
