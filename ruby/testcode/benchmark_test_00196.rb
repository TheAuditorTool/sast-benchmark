require_relative 'shared'

def aws_upload_handler(req)
  secret_key = Rails.application.credentials.dig(:aws, :secret_access_key)
  client = Aws::S3::Client.new(
    region: 'us-east-1',
    access_key_id: Rails.application.credentials.dig(:aws, :access_key_id),
    secret_access_key: secret_key
  )
  client.put_object(bucket: req[:bucket], key: req[:key], body: req[:data])
  BenchmarkResponse.ok('uploaded')
end
