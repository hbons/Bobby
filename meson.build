project(
  'Bobby',
  'rust',
  version: '49.0.0',
  meson_version: '>=1.1'
)

command_name = 'bobby'
configuration = configuration_data()
configuration.set('APP_ID', 'studio.planetpeanut.Bobby')


gnome = import('gnome')
dependency('glib-2.0')
dependency('gtk4')
dependency('libadwaita-1')

devenv = environment()
devenv.prepend('PATH', meson.current_build_dir())
meson.add_devenv(devenv)

subdir('data')
subdir('data/icons')
subdir('data/screenshots')
subdir('src')


cargo = find_program('cargo', required: true)
cargo_sources = files(
  'Cargo.toml',
  'Cargo.lock',
)

cargo_build = custom_target(
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

summary(
  {
    'nightly': get_option('nightly'),
    'bindir': get_option('bindir')
  },
  section: 'Options',
)
