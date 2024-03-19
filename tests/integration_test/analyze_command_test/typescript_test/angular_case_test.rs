use crate::integration_test::assert_analyzed_source_code;
use indoc::indoc;

#[test]
fn test_angular_code() {
    let source_code = indoc! {r#"
    export const enum JitCompilerUsage {
        Decorator,
        PartialDeclaration,
    }

    export interface JitCompilerUsageRequest {
        usage: JitCompilerUsage;
        kind: 'directive'|'component'|'pipe'|'injectable'|'NgModule';
        type: Type;
    }

    export function getCompilerFacade(request: JitCompilerUsageRequest): CompilerFacade {
        const globalNg: ExportedCompilerFacade = global['ng'];
        if (globalNg && globalNg.ɵcompilerFacade) {
            return globalNg.ɵcompilerFacade;
        }

        if (typeof ngDevMode === 'undefined' || ngDevMode) {
            console.error(`JIT compilation failed for ${request.kind}`, request.type);

            let message = `The ${request.kind} '${
                request
                    .type.name}' needs to be compiled using the JIT compiler, but '@angular/compiler' is not available.\n\n`;
            if (request.usage === JitCompilerUsage.PartialDeclaration) {
            message += `The ${request.kind} is part of a library that has been partially compiled.\n`;
            message +=
                `However, the Angular Linker has not processed the library such that JIT compilation is used as fallback.\n`;
            message += '\n';
            message +=
                `Ideally, the library is processed using the Angular Linker to become fully AOT compiled.\n`;
            } else {
            message +=
                `JIT compilation is discouraged for production use-cases! Consider using AOT mode instead.\n`;
            }
            message +=
                `Alternatively, the JIT compiler should be loaded by bootstrapping using '@angular/platform-browser-dynamic' or '@angular/platform-server',\n`;
            message +=
                `or manually provide the compiler with 'import "@angular/compiler";' before bootstrapping.`;
            throw new Error(message);
        } else {
            throw new Error('JIT compiler unavailable');
        }
    }"#};

    let expected = indoc! {r#"
    // [TODO] JitCompilerUsage
    export const enum JitCompilerUsage {
        Decorator,
        PartialDeclaration,
    }

    // [TODO] JitCompilerUsageRequest
    export interface JitCompilerUsageRequest {
        usage: JitCompilerUsage;
        kind: 'directive'|'component'|'pipe'|'injectable'|'NgModule';
        type: Type;
    }

    // [TODO] getCompilerFacade
    export function getCompilerFacade(request: JitCompilerUsageRequest): CompilerFacade {
        const globalNg: ExportedCompilerFacade = global['ng'];
        if (globalNg && globalNg.ɵcompilerFacade) {
            return globalNg.ɵcompilerFacade;
        }

        if (typeof ngDevMode === 'undefined' || ngDevMode) {
            console.error(`JIT compilation failed for ${request.kind}`, request.type);

            let message = `The ${request.kind} '${
                request
                    .type.name}' needs to be compiled using the JIT compiler, but '@angular/compiler' is not available.\n\n`;
            if (request.usage === JitCompilerUsage.PartialDeclaration) {
            message += `The ${request.kind} is part of a library that has been partially compiled.\n`;
            message +=
                `However, the Angular Linker has not processed the library such that JIT compilation is used as fallback.\n`;
            message += '\n';
            message +=
                `Ideally, the library is processed using the Angular Linker to become fully AOT compiled.\n`;
            } else {
            message +=
                `JIT compilation is discouraged for production use-cases! Consider using AOT mode instead.\n`;
            }
            message +=
                `Alternatively, the JIT compiler should be loaded by bootstrapping using '@angular/platform-browser-dynamic' or '@angular/platform-server',\n`;
            message +=
                `or manually provide the compiler with 'import "@angular/compiler";' before bootstrapping.`;
            throw new Error(message);
        } else {
            throw new Error('JIT compiler unavailable');
        }
    }"#};

    assert_analyzed_source_code(source_code, expected, "typescript")
}

#[test]
#[ignore = "Should not be add TODO comment to above the `export ... from` statement"]
fn test_angular_import_statement() {
    let source_code = indoc! {r#"
        import {global} from '../util/global';
        import {CompilerFacade, ExportedCompilerFacade, Type} from './compiler_facade_interface';
        export * from './compiler_facade_interface';
    "#};

    let _should_fix_output = indoc! {r#"
    import {global} from '../util/global';
    import {CompilerFacade, ExportedCompilerFacade, Type} from './compiler_facade_interface';
    // [TODO] anonymous
    export * from './compiler_facade_interface';"#};

    assert_analyzed_source_code(source_code, source_code, "typescript")
}

#[test]
fn test_abstract_class_statement() {
    let source_code = indoc! {r#"
    export abstract class RendererFactory2 {
        abstract createRenderer(hostElement: any, type: RendererType2|null): Renderer2;
        abstract begin?(): void;
        abstract end?(): void;
        abstract whenRenderingDone?(): Promise<any>;
    }"#};

    let exptected = indoc! {r#"
    // [TODO] RendererFactory2
    export abstract class RendererFactory2 {
        abstract createRenderer(hostElement: any, type: RendererType2|null): Renderer2;
        abstract begin?(): void;
        abstract end?(): void;
        abstract whenRenderingDone?(): Promise<any>;
    }"#};

    assert_analyzed_source_code(source_code, exptected, "typescript")
}

#[test]
fn test_normal_class_statement() {
    let source_code = indoc! {r#"
    export class TransferState {
        static ɵprov =
            ɵɵdefineInjectable({
                token: TransferState,
                providedIn: 'root',
                factory: initTransferState,
            });

        /** @internal */
        store: Record<string, unknown|undefined> = {};

        private onSerializeCallbacks: {[k: string]: () => unknown | undefined} = {};

        /**
         * Get the value corresponding to a key. Return `defaultValue` if key is not found.
         */
        get<T>(key: StateKey<T>, defaultValue: T): T {
            return this.store[key] !== undefined ? this.store[key] as T : defaultValue;
        }

        /**
         * Set the value corresponding to a key.
         */
        set<T>(key: StateKey<T>, value: T): void {
            this.store[key] = value;
        }

        /**
         * Remove a key from the store.
         */
        remove<T>(key: StateKey<T>): void {
            delete this.store[key];
        }

        /**
         * Test whether a key exists in the store.
         */
        hasKey<T>(key: StateKey<T>): boolean {
            return this.store.hasOwnProperty(key);
        }

        /**
         * Indicates whether the state is empty.
         */
        get isEmpty(): boolean {
            return Object.keys(this.store).length === 0;
        }

        /**
         * Register a callback to provide the value for a key when `toJson` is called.
         */
        onSerialize<T>(key: StateKey<T>, callback: () => T): void {
            this.onSerializeCallbacks[key] = callback;
        }
    }"#};

    let expected = indoc! {r#"
    // [TODO] TransferState
    export class TransferState {
        static ɵprov =
            ɵɵdefineInjectable({
                token: TransferState,
                providedIn: 'root',
                factory: initTransferState,
            });

        /** @internal */
        store: Record<string, unknown|undefined> = {};

        private onSerializeCallbacks: {[k: string]: () => unknown | undefined} = {};

        /**
         * Get the value corresponding to a key. Return `defaultValue` if key is not found.
         */
        get<T>(key: StateKey<T>, defaultValue: T): T {
            return this.store[key] !== undefined ? this.store[key] as T : defaultValue;
        }

        /**
         * Set the value corresponding to a key.
         */
        set<T>(key: StateKey<T>, value: T): void {
            this.store[key] = value;
        }

        /**
         * Remove a key from the store.
         */
        remove<T>(key: StateKey<T>): void {
            delete this.store[key];
        }

        /**
         * Test whether a key exists in the store.
         */
        hasKey<T>(key: StateKey<T>): boolean {
            return this.store.hasOwnProperty(key);
        }

        /**
         * Indicates whether the state is empty.
         */
        get isEmpty(): boolean {
            return Object.keys(this.store).length === 0;
        }

        /**
         * Register a callback to provide the value for a key when `toJson` is called.
         */
        onSerialize<T>(key: StateKey<T>, callback: () => T): void {
            this.onSerializeCallbacks[key] = callback;
        }
    }"#};

    assert_analyzed_source_code(source_code, expected, "typescript")
}
