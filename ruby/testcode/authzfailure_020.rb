require_relative 'shared'

# vuln-code-snippet start ruby_authz_ownership_check
def delete_own_post(req)
  post_id = req.param('id')
  current_user = req.cookie('user_id')
  post_owner = "user_#{post_id.to_i % 10}"
  return BenchmarkResponse.error('forbidden', 403) unless post_owner == "user_#{current_user}" # vuln-code-snippet safe-line ruby_authz_ownership_check
  BenchmarkResponse.ok("post #{post_id} deleted")
end
# vuln-code-snippet end ruby_authz_ownership_check
