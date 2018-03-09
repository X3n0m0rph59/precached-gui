Name:    precached-gui
Version: 0.1.0
Release: 2%{?dist}
Summary: precached-gui - A GUI for precached
URL:     https://x3n0m0rph59.github.io/precached/
License: GPLv3+

# Source0: https://github.com/X3n0m0rph59/precached-gui.git
Source0: https://github.com/X3n0m0rph59/%{name}/archive/master.tar.gz

BuildRoot: %{_tmppath}/%{name}-build

BuildRequires: gtk3-devel
BuildRequires: glib2-devel
BuildRequires: atk-devel
BuildRequires: cairo-devel
BuildRequires: gdk-pixbuf2-devel
BuildRequires: pango-devel
BuildRequires: systemd
BuildRequires: dbus-devel
BuildRequires: zeromq-devel
BuildRequires: cargo

Requires: gtk3 gdk-pixbuf2 dbus zeromq

%global gittag master
%global debug_package %{nil}

%description
A GTK+ based GUI for precached.

%prep
%autosetup -n %{name}-%{version}

%build
cargo build --all --release --verbose

%install
%{__mkdir_p} %{buildroot}%{_datarootdir}/metainfo/
%{__mkdir_p} %{buildroot}/%{_datarootdir}/icons/hicolor/scalable/
cp -a %{_builddir}/%{name}-%{version}/support/appstream/org.precache.precached-gui.appdata.xml %{buildroot}/%{_datarootdir}/metainfo/
cp -a %{_builddir}/%{name}-%{version}/support/assets/precached.svg %{buildroot}/%{_datarootdir}/icons/hicolor/scalable/precached.svg
install -Dp -m 0755 %{_builddir}/%{name}-%{version}/target/release/precached-gui %{buildroot}%{_bindir}/precached-gui

%files
%license LICENSE
%{_bindir}/precached-gui
%{_datarootdir}/icons/hicolor/scalable/precached.svg
%{_datarootdir}/metainfo/org.precache.precached-gui.appdata.xml

%changelog
* Fri Mar 09 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-2
- rebuilt

* Fri Mar 09 2018 X3n0m0rph59 <x3n0m0rph59@gmail.com> - 0.1.0-1
- rebuilt

