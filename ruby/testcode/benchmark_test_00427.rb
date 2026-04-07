require_relative 'shared'
require 'securerandom'

begin
  require 'aws-sdk-s3'
rescue LoadError
end

S3_BUCKET        = ENV.fetch('S3_BUCKET', 'my-uploads-bucket')
S3_REGION        = ENV.fetch('S3_REGION', 'us-east-1')
PRESIGN_EXPIRES  = 300

def generate_presigned_upload_url(req)
  content_type = req.param('content_type')
  return BenchmarkResponse.bad_request('content_type required') if content_type.empty?

  object_key = "uploads/#{SecureRandom.uuid}"

  presigned_url = if defined?(Aws::S3::Presigner)
                    signer = Aws::S3::Presigner.new(
                      client: Aws::S3::Client.new(region: S3_REGION)
                    )
                    signer.presigned_url(
                      :put_object,
                      bucket:       S3_BUCKET,
                      key:          object_key,
                      expires_in:   PRESIGN_EXPIRES,
                      content_type: content_type
                    )
                  else
                    "https://#{S3_BUCKET}.s3.#{S3_REGION}.amazonaws.com/#{object_key}?X-Amz-Expires=#{PRESIGN_EXPIRES}"
                  end

  BenchmarkResponse.json({ upload_url: presigned_url, key: object_key })
end
