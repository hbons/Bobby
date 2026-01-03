apps_dir = join_paths(get_option('prefix'), 'share', 'applications')

if get_option('nightly')
  install_data(
    sources: 'studio.planetpeanut.Bobby.Nightly.desktop',
    install_dir: apps_dir,
    rename: 'studio.planetpeanut.Bobby.desktop'
  )
else
  install_data(
    sources: 'studio.planetpeanut.Bobby.desktop',
    install_dir: apps_dir
  )
endif

install_data(
  'studio.planetpeanut.Bobby.metainfo.xml',
  install_dir: join_paths(get_option('datadir'), 'metainfo')
)

install_data(
  'studio.planetpeanut.Bobby-mime.xml',
  install_dir: join_paths(get_option('datadir'), 'mime', 'packages')
)


install_data(
  'studio.planetpeanut.Bobby.gschema.xml',
  install_dir: join_paths(get_option('datadir'), 'glib-2.0', 'schemas')
)

# Compile schemas locally, so we can run uninstalled under a devenv
gnome.compile_schemas()

compile_schemas = find_program('glib-compile-schemas', required: false)
if compile_schemas.found()
  test('Validate schema file',
    compile_schemas,
    args: ['--strict', '--dry-run', meson.current_source_dir()],
    suite: ['lint'],
  )
endif
