require_relative 'shared'

def s3_list_handler(req)
  client = Aws::S3::Client.new
  objects = client.list_objects_v2(bucket: req[:bucket])
  BenchmarkResponse.json({ keys: objects.contents.map(&:key) })
end
