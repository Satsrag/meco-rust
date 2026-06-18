Pod::Spec.new do |s|
  s.name         = 'Meco'
  s.version      = '0.1.0'
  s.summary      = 'Mongolian Encoding Converter — Rust core via UniFFI.'
  s.description  = 'Convert between Mongolian encodings (Zvvnmod, Z52, Menk-shape/letter, Delehi). '\
                   'Backed by the shared Rust meco core, verified byte-exact against the original Java.'
  s.homepage     = 'https://github.com/zvvnmod/meco-rust'
  s.license      = { :type => 'Apache-2.0' }
  s.author       = 'zvvnmod'
  s.platform     = :ios, '13.0'
  s.swift_version = '5.9'

  # The release CI uploads Meco.xcframework.zip (the framework) alongside the generated Swift wrapper.
  s.source                = { :http => "https://github.com/zvvnmod/meco-rust/releases/download/v#{s.version}/Meco.xcframework.zip" }
  s.vendored_frameworks   = 'MecoFFI.xcframework'
  s.source_files          = 'Sources/Meco/*.swift'
end
