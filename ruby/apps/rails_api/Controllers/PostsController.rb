require_relative '../../../testcode/shared'
require 'securerandom'

# Rails API - PostsController
# Covers: sqli, xss, massassign, fileupload

# vuln-code-snippet start ra_sqli_ar_string
def posts_search(req)
  title = req.param('title')
  results = FakeActiveRecord::Base.where("title LIKE '%#{title}%'") # vuln-code-snippet vuln-line ra_sqli_ar_string
  BenchmarkResponse.json({ posts: results.to_a })
end
# vuln-code-snippet end ra_sqli_ar_string

# vuln-code-snippet start ra_sqli_ar_param
def posts_search_safe(req)
  title = req.param('title')
  results = FakeActiveRecord::Base.where(title: title) # vuln-code-snippet safe-line ra_sqli_ar_param
  BenchmarkResponse.json({ posts: results.to_a })
end
# vuln-code-snippet end ra_sqli_ar_param

# vuln-code-snippet start ra_sqli_raw_execute
def posts_by_author(req)
  author = req.param('author')
  sql = "SELECT * FROM posts WHERE author = '#{author}'"
  rows = FakeActiveRecord::Base.connection.execute(sql) # vuln-code-snippet vuln-line ra_sqli_raw_execute
  BenchmarkResponse.json({ posts: rows.to_a })
end
# vuln-code-snippet end ra_sqli_raw_execute

# vuln-code-snippet start ra_sqli_find_safe
def posts_show(req)
  id = req.param('id')
  post = FakeActiveRecord::Base.find(id.to_i) # vuln-code-snippet safe-line ra_sqli_find_safe
  BenchmarkResponse.json({ post: post.to_a.first })
end
# vuln-code-snippet end ra_sqli_find_safe

# vuln-code-snippet start ra_xss_html_safe
def posts_render_title(req)
  title = req.param('title')
  author = req.param('author')
  html = "<h1>#{title.html_safe}</h1><p>By #{author}</p>" # vuln-code-snippet vuln-line ra_xss_html_safe
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ra_xss_html_safe

# vuln-code-snippet start ra_xss_sanitize
def posts_render_safe(req)
  title = req.param('title')
  author = req.param('author')
  html = "<h1>#{escape_html(title)}</h1><p>By #{escape_html(author)}</p>" # vuln-code-snippet safe-line ra_xss_sanitize
  BenchmarkResponse.html(html)
end
# vuln-code-snippet end ra_xss_sanitize

# vuln-code-snippet start ra_massassign_permit_bang
def posts_create(req)
  params = FakeParams.new(req.post_data)
  permitted = params.permit! # vuln-code-snippet vuln-line ra_massassign_permit_bang
  post = FakeActiveRecord::Base.where(permitted.to_h)
  BenchmarkResponse.json({ post: post.to_a.first })
end
# vuln-code-snippet end ra_massassign_permit_bang

# vuln-code-snippet start ra_massassign_permit
def posts_update(req)
  params = FakeParams.new(req.post_data)
  permitted = params.permit(:title, :body, :category) # vuln-code-snippet safe-line ra_massassign_permit
  post = FakeActiveRecord::Base.where(permitted.to_h)
  BenchmarkResponse.json({ post: post.to_a.first })
end
# vuln-code-snippet end ra_massassign_permit

# vuln-code-snippet start ra_fileupload_no_validate
def posts_upload(req)
  upload = req.file('attachment')
  original_name = upload[:filename]
  file_data = upload[:data]
  File.write("/uploads/posts/#{original_name}", file_data) # vuln-code-snippet vuln-line ra_fileupload_no_validate
  BenchmarkResponse.json({ filename: original_name, status: 'uploaded' })
end
# vuln-code-snippet end ra_fileupload_no_validate

# vuln-code-snippet start ra_fileupload_validated
def posts_upload_safe(req)
  upload = req.file('attachment')
  original_name = upload[:filename]
  file_data = upload[:data]
  allowed_exts = %w[.jpg .jpeg .png .gif .pdf]
  ext = File.extname(original_name).downcase
  return BenchmarkResponse.bad_request('invalid file type') unless allowed_exts.include?(ext) # vuln-code-snippet safe-line ra_fileupload_validated
  safe_name = "#{SecureRandom.uuid}#{ext}"
  File.write("/uploads/posts/#{safe_name}", file_data)
  BenchmarkResponse.json({ filename: safe_name, status: 'uploaded' })
end
# vuln-code-snippet end ra_fileupload_validated
