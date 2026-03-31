require_relative '../../testcode/shared'
begin; require 'sqlite3'; rescue LoadError; end

# vuln-code-snippet start si_sqli_concat
def si_sqli_concat(req)
  db = get_db_connection
  username = req.param('username')
  rows = db.execute("SELECT * FROM users WHERE username = '" + username + "'")  # vuln-code-snippet vuln-line si_sqli_concat
  user = rows.first
  return BenchmarkResponse.bad_request('not found') unless user
  BenchmarkResponse.json({ id: user[0], name: user[1] })
end
# vuln-code-snippet end si_sqli_concat

# vuln-code-snippet start si_sqli_param
def si_sqli_param(req)
  db = get_db_connection
  username = req.param('username')
  rows = db.execute("SELECT * FROM users WHERE username = ?", [username])  # vuln-code-snippet safe-line si_sqli_param
  user = rows.first
  return BenchmarkResponse.bad_request('not found') unless user
  BenchmarkResponse.json({ id: user[0], name: user[1] })
end
# vuln-code-snippet end si_sqli_param

# vuln-code-snippet start si_xss_reflect
def si_xss_reflect(req)
  name = req.param('name')
  body = "<h1>Hello, #{name}!</h1>"  # vuln-code-snippet vuln-line si_xss_reflect
  BenchmarkResponse.html(body)
end
# vuln-code-snippet end si_xss_reflect

# vuln-code-snippet start si_xss_escape
def si_xss_escape(req)
  name = req.param('name')
  safe_name = escape_html(name)  # vuln-code-snippet safe-line si_xss_escape
  body = "<h1>Hello, #{safe_name}!</h1>"
  BenchmarkResponse.html(body)
end
# vuln-code-snippet end si_xss_escape

# vuln-code-snippet start si_redirect_open
def si_redirect_open(req)
  next_url = req.param('next')
  BenchmarkResponse.redirect(next_url)  # vuln-code-snippet vuln-line si_redirect_open
end
# vuln-code-snippet end si_redirect_open

# vuln-code-snippet start si_redirect_path
def si_redirect_path(req)
  next_url = req.param('next')
  if next_url.start_with?('/') && !next_url.start_with?('//')  # vuln-code-snippet safe-line si_redirect_path
    BenchmarkResponse.redirect(next_url)
  else
    BenchmarkResponse.bad_request('redirect target must be a relative path')
  end
end
# vuln-code-snippet end si_redirect_path
