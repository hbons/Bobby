project(
  'Bobby',
  'rust',
  version: '50.0.0',
  meson_version: '>=1.1'
)

command_name = 'bobby'
application_id = 'studio.planetpeanut.Bobby'


gnome = import('gnome')
dependency('glib-2.0')
dependency('gtk4')
dependency('libadwaita-1')


subdir('data')
subdir('data/icons')

summary(
  {
    'nightly': get_option('nightly'),
    'bindir': get_option('bindir')
  },
  section: 'Options',
)


cargo = find_program('cargo', required: true)

custom_target(
  command_name,
  command: [
    'sh', '-c',
    'cd ' + meson.project_source_root() + ' && ' +
    'cargo build --release && ' +
    'cp target/release/' + command_name + ' ' + meson.project_build_root() + '/' + command_name
  ],
  output: command_name,
  build_by_default: true,
  console: true,
  install: true,
  install_dir: get_option('bindir'),
)

test('cargo_test',
  cargo,
  args: ['test'],
  workdir: meson.project_source_root(),
  timeout: 300,
)


gnome.post_install(
    glib_compile_schemas: true,
    gtk_update_icon_cache: true,
    update_desktop_database: true,
)

meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  meson.project_source_root(),
  meson.project_build_root() / 'meson-dist' / '@0@-@1@'.format(meson.project_name(), meson.project_version()),
)
