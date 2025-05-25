use crate::parsers::yaml;

#[derive(Debug)]
pub enum PubspecYamlError {
    DocumentNotAMap,
    InvalidVersionFieldDataType,
    MissingVersionField,
    ParseYml(Box<marked_yaml::LoadError>),
    ReplaceYamlValue(yaml::NodeReplaceError),
}

impl core::error::Error for PubspecYamlError {}

impl core::fmt::Display for PubspecYamlError {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DocumentNotAMap => write!(f, "Input document is not a map"),
            Self::InvalidVersionFieldDataType => write!(f, "\"version\" field is not a string"),
            Self::MissingVersionField => write!(f, "\"version\" field not found"),
            Self::ParseYml(error) => error.fmt(f),
            Self::ReplaceYamlValue(error) => error.fmt(f),
        }
    }
}

#[inline]
pub fn set_pubspec_version(
    mut contents: String,
    version: &str,
) -> Result<(bool, String), PubspecYamlError> {
    let document =
        yaml::parse(&contents).map_err(|error| PubspecYamlError::ParseYml(Box::new(error)))?;

    let map = document
        .as_mapping()
        .ok_or(PubspecYamlError::DocumentNotAMap)?;

    let version_node = map
        .get_node("version")
        .ok_or(PubspecYamlError::MissingVersionField)?;

    let scalar = version_node
        .as_scalar()
        .ok_or(PubspecYamlError::InvalidVersionFieldDataType)?;

    let output = yaml::replace_node_value_in_input(&contents, scalar, version)
        .map_err(PubspecYamlError::ReplaceYamlValue)?;

    let modified = output != contents;

    contents = output;

    let output = if modified {
        yaml::serialize(&contents)
    } else {
        contents
    };

    Ok((modified, output))
}

#[inline]
pub const fn update_lock_files(_dir: &std::path::Path) -> bool {
    true
}

#[cfg(test)]
mod test_set_pubspec_version {
    use super::{PubspecYamlError, set_pubspec_version};
    use crate::package_managers::error::PackageManagerError;

    const INPUT: &str = r#"name: someapplication
description: A new Flutter project.
# The following line prevents the package from being accidentally published to
# pub.dev using `flutter pub publish`. This is preferred for private packages.
publish_to: 'none' # Remove this line if you wish to publish to pub.dev

# The following defines the version and build number for your application.
# A version number is three numbers separated by dots, like 1.2.43
# followed by an optional build number separated by a +.
# Both the version and the builder number may be overridden in flutter
# build by specifying --build-name and --build-number, respectively.
# In Android, build-name is used as versionName while build-number used as versionCode.
# Read more about Android versioning at https://developer.android.com/studio/publish/versioning
# In iOS, build-name is used as CFBundleShortVersionString while build-number is used as CFBundleVersion.
# Read more about iOS versioning at
# https://developer.apple.com/library/archive/documentation/General/Reference/InfoPlistKeyReference/Articles/CoreFoundationKeys.html
# In Windows, build-name is used as the major, minor, and patch parts
# of the product and file versions while build-number is used as the build suffix.
version: 1.0.7+21

environment:
  sdk: '>=3.0.6 <4.0.0'

flutter_launcher_icons:
  image_path: 'assets/images/launch_icon.png'
  android: true
  ios: true

# Dependencies specify other packages that your package needs in order to work.
# To automatically upgrade your package dependencies to the latest versions
# consider running `flutter pub upgrade --major-versions`. Alternatively,
# dependencies can be manually updated by changing the version numbers below to
# the latest version available on pub.dev. To see which dependencies have newer
# versions available, run `flutter pub outdated`.
dependencies:
  flutter:
    sdk: flutter
  permission_handler: ^10.0.0  # eller den nyeste versjonen

  # The following adds the Cupertino Icons font to your application.
  # Use with the CupertinoIcons class for iOS style icons.
  cupertino_icons: ^1.0.2
  auto_route: 7.8.4
  firebase_core: ^2.17.0
  json_annotation: ^4.6.0
  flutter_riverpod: ^2.4.3
  cloud_firestore: ^4.9.3
  firebase_auth: ^4.10.1
  flutter_svg: ^2.0.7
  collection: ^1.17.2
  intl: ^0.19.0
  firebase_messaging: ^14.7.2
  flutter_launcher_icons: ^0.13.1
  speech_to_text: ^6.5.1

dev_dependencies:
  json_serializable: ^6.3.1
  flutter_test:
    sdk: flutter

  # The "flutter_lints" package below contains a set of recommended lints to
  # encourage good coding practices. The lint set provided by the package is
  # activated in the `analysis_options.yaml` file located at the root of your
  # package. See that file for information about deactivating specific lint
  # rules and activating additional ones.
  flutter_lints: ^3.0.1
  auto_route_generator: ^7.0.0
  build_runner: ^2.4.4

# For information on the generic Dart part of this file, see the
# following page: https://dart.dev/tools/pub/pubspec

# The following section is specific to Flutter packages.
flutter:
  # The following line ensures that the Material Icons font is
  # included with your application, so that you can use the icons in
  # the material Icons class.
  uses-material-design: true
  assets:
    - assets/
    - assets/images/

  # To add assets to your application, add an assets section, like this:
  # assets:
  #   - images/a_dot_burr.jpeg
  #   - images/a_dot_ham.jpeg

  # An image asset can refer to one or more resolution-specific "variants", see
  # https://flutter.dev/assets-and-images/#resolution-aware

  # For details regarding adding assets from package dependencies, see
  # https://flutter.dev/assets-and-images/#from-packages

  # To add custom fonts to your application, add a fonts section here,
  # in this "flutter" section. Each entry in this list should have a
  # "family" key with the font family name, and a "fonts" key with a
  # list giving the asset and other descriptors for the font. For
  # example:
  # fonts:
  #   - family: Schyler
  #     fonts:
  #       - asset: fonts/Schyler-Regular.ttf
  #       - asset: fonts/Schyler-Italic.ttf
  #         style: italic
  #   - family: Trajan Pro
  #     fonts:
  #       - asset: fonts/TrajanPro.ttf
  #       - asset: fonts/TrajanPro_Bold.ttf
  #         weight: 700
  #
  # For details regarding fonts from package dependencies,
  # see https://flutter.dev/custom-fonts/#from-packages
"#;

    #[test]
    fn it_should_update_version() {
        let version = format!(
            "{}.{}.{}",
            rand::random::<u16>(),
            rand::random::<u16>(),
            rand::random::<u16>()
        );

        let new_version_line = format!("version: {version}");

        let expected_output = INPUT.replace("version: 1.0.7+21", &new_version_line);

        assert!(expected_output.contains(&new_version_line));

        let (modified, output) =
            set_pubspec_version(INPUT.to_string(), &version).expect("it not to raise");

        assert!(modified);

        assert_eq!(output, expected_output);

        // Validate we do not modify file if version is the same
        {
            let (modified, output) =
                set_pubspec_version(output, &version).expect("it not to raise");

            assert!(!modified);

            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn it_support_multiline_strings() {
        let input = INPUT.replace("version: 1.0.7+21", "version:\n   1.0.7+21");

        let version = format!(
            "{}.{}.{}",
            rand::random::<u16>(),
            rand::random::<u16>(),
            rand::random::<u16>()
        );

        let new_version_line = format!("version:\n   {version}");

        let expected_output = input.replace("version:\n   1.0.7+21", &new_version_line);

        assert!(expected_output.contains(&new_version_line));

        let (modified, output) = set_pubspec_version(input, &version).expect("it not to raise");

        assert!(modified);

        assert_eq!(output, expected_output);

        // Validate we do not modify file if version is the same
        {
            let (modified, output) =
                set_pubspec_version(output, &version).expect("it not to raise");

            assert!(!modified);

            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn it_should_require_version_field() {
        let input = "name: Mads\n";

        let result = set_pubspec_version(input.to_string(), "1.23.4")
            .expect_err("it should return an error");

        assert!(matches!(result, PubspecYamlError::MissingVersionField));

        assert!(
            crate::error::Error::from(PackageManagerError::from(result))
                .to_string()
                .contains("\"version\"")
        );
    }

    #[test]
    fn version_field_should_be_string() {
        let input = "version:\n  - value1\n";

        let result = set_pubspec_version(input.to_string(), "1.23.4")
            .expect_err("it should return an error");

        assert!(matches!(
            result,
            PubspecYamlError::InvalidVersionFieldDataType
        ));

        assert!(
            crate::error::Error::from(PackageManagerError::from(result))
                .to_string()
                .contains("\"version\"")
        );
    }
}
