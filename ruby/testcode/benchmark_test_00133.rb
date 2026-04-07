require_relative 'shared'

def download_file_safe(req)
  file_id = req.param('file_id')
  current_user = req.cookie('user_id')
  owner = "user_#{current_user}"
  path = "/uploads/#{owner}/#{file_id}"
  return BenchmarkResponse.error('forbidden', 403) unless path.start_with?("/uploads/#{owner}/")
  BenchmarkResponse.ok("downloading: #{path}")
end
