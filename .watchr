def compile
  cmd = IO.popen(ENV, 'cargo build')
  print cmd.getc until cmd.eof?
  cmd.close
  $?.to_i == 0
end

def test
  cmd = IO.popen(ENV, 'cargo test')
  print cmd.getc until cmd.eof?
  cmd.close
  $?.to_i == 0
end

watch('^(src|tests)/.*') { compile && test }
