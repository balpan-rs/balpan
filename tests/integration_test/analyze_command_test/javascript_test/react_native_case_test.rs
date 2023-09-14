use indoc::indoc;

use crate::integration_test::assert_analyzed_source_code;

#[test]
fn test_function_declaration() {
    let source_code = indoc! {r#"
    function getPackageName(file /*: string */) /*: string */ {
        return path.relative(PACKAGES_DIR, file).split(path.sep)[0];
    }

    function getBuildPath(file /*: string */) /*: string */ {
        const packageDir = path.join(PACKAGES_DIR, getPackageName(file));

        return path.join(
            packageDir,
            file.replace(path.join(packageDir, SRC_DIR), BUILD_DIR),
        );
    }

    async function rewritePackageExports(packageName /*: string */) {
        const packageJsonPath = path.join(PACKAGES_DIR, packageName, 'package.json');
        const pkg = JSON.parse(await fs.readFile(packageJsonPath, 'utf8'));

        await fs.writeFile(
            packageJsonPath,
            prettier.format(JSON.stringify(pkg), {parser: 'json'}),
        );
    }"#};

    let expected = indoc! {r#"
    // [TODO] getPackageName
    function getPackageName(file /*: string */) /*: string */ {
        return path.relative(PACKAGES_DIR, file).split(path.sep)[0];
    }

    // [TODO] getBuildPath
    function getBuildPath(file /*: string */) /*: string */ {
        const packageDir = path.join(PACKAGES_DIR, getPackageName(file));

        return path.join(
            packageDir,
            file.replace(path.join(packageDir, SRC_DIR), BUILD_DIR),
        );
    }

    // [TODO] rewritePackageExports
    async function rewritePackageExports(packageName /*: string */) {
        const packageJsonPath = path.join(PACKAGES_DIR, packageName, 'package.json');
        const pkg = JSON.parse(await fs.readFile(packageJsonPath, 'utf8'));

        await fs.writeFile(
            packageJsonPath,
            prettier.format(JSON.stringify(pkg), {parser: 'json'}),
        );
    }"#};

    assert_analyzed_source_code(source_code, expected, "javascript")
}

#[test]
fn test_class() {
    let source_code = indoc! {r#"
    export class KeyPressHandler {
        _isInterceptingKeyStrokes = false;
        _isHandlingKeyPress = false;
        _onPress: (key: string) => Promise<void>;

        constructor(onPress: (key: string) => Promise<void>) {
            this._onPress = onPress;
        }

        /** Start intercepting all key strokes and passing them to the input `onPress` method. */
        startInterceptingKeyStrokes() {
            if (this._isInterceptingKeyStrokes) {
                return;
            }
            this._isInterceptingKeyStrokes = true;
            const {stdin} = process;
            // $FlowFixMe[prop-missing]
            stdin.setRawMode(true);
            stdin.resume();
            stdin.setEncoding('utf8');
            stdin.on('data', this._handleKeypress);
        }

        /** Stop intercepting all key strokes. */
        stopInterceptingKeyStrokes() {
            if (!this._isInterceptingKeyStrokes) {
                return;
            }
            this._isInterceptingKeyStrokes = false;
            const {stdin} = process;
            stdin.removeListener('data', this._handleKeypress);
            // $FlowFixMe[prop-missing]
            stdin.setRawMode(false);
            stdin.resume();
        }
    }"#};

    let expected = indoc! {r#"
    // [TODO] KeyPressHandler
    export class KeyPressHandler {
        _isInterceptingKeyStrokes = false;
        _isHandlingKeyPress = false;
        _onPress: (key: string) => Promise<void>;

        constructor(onPress: (key: string) => Promise<void>) {
            this._onPress = onPress;
        }

        /** Start intercepting all key strokes and passing them to the input `onPress` method. */
        startInterceptingKeyStrokes() {
            if (this._isInterceptingKeyStrokes) {
                return;
            }
            this._isInterceptingKeyStrokes = true;
            const {stdin} = process;
            // $FlowFixMe[prop-missing]
            stdin.setRawMode(true);
            stdin.resume();
            stdin.setEncoding('utf8');
            stdin.on('data', this._handleKeypress);
        }

        /** Stop intercepting all key strokes. */
        stopInterceptingKeyStrokes() {
            if (!this._isInterceptingKeyStrokes) {
                return;
            }
            this._isInterceptingKeyStrokes = false;
            const {stdin} = process;
            stdin.removeListener('data', this._handleKeypress);
            // $FlowFixMe[prop-missing]
            stdin.setRawMode(false);
            stdin.resume();
        }
    }"#};

    assert_analyzed_source_code(source_code, expected, "javascript")
}