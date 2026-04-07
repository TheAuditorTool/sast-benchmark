require_relative 'shared'
require 'stringio'

# Minimal stand-in for a Paperclip-style attachment handler.
class PaperclipStyleAttachment
  attr_reader :content_type, :original_filename, :data

  def initialize
    @content_type      = nil
    @original_filename = nil
    @data              = nil
  end

  def assign(upload_io)
    @original_filename = upload_io[:filename]
    @content_type      = upload_io[:content_type]
    @data              = upload_io[:data]
  end

  def save(dest_dir)
    path = File.join(dest_dir, @original_filename)
    File.write(path, @data)
    path
  end
end

# vuln-code-snippet start ruby_fileupload_gem_paperclip_old
def upload_paperclip_style(req)
  upload     = req.file('avatar')
  return BenchmarkResponse.bad_request('no file') unless upload

  attachment = PaperclipStyleAttachment.new
  attachment.assign(upload) # vuln-code-snippet vuln-line ruby_fileupload_gem_paperclip_old

  saved_path = attachment.save('/var/uploads')
  BenchmarkResponse.ok("saved to #{saved_path}")
end
# vuln-code-snippet end ruby_fileupload_gem_paperclip_old
