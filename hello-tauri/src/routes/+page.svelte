<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { open as openFileDialog } from "@tauri-apps/plugin-dialog";

    let documentText = "";
    let cursorPosition = 0;
    let saved_state = "saved";
    let document_name = "untitled.txt";
    let openFilePath = "";

    function updateCursor(event: Event) {
        const editor = event.currentTarget as HTMLTextAreaElement;
        cursorPosition = editor.selectionStart;
    }

    async function open() {
        const filePath = await openFileDialog({
            multiple: false,
            directory: false,
        });

        if (!filePath || Array.isArray(filePath)) {
            return;
        }
        openFilePath = filePath;
        document_name =
            filePath.split("/").pop().split("\\").pop() || "untitled.txt";

        invoke<string>("open", { filePath }).then((message) => {
            documentText = message;
            cursorPosition = 0;
            saved_state = "saved";
        });
    }

    function save() {
        invoke("save", { filePath: openFilePath, contents: documentText })
            .catch((e) => (saved_state = e))
            .then(() => {
                saved_state = "saved";
            });
    }
</script>

<main class="min-h-screen bg-zinc-950 text-zinc-100">
    <section class="flex min-h-screen flex-col">
        <header
            class="border-b border-zinc-800 px-4 py-3 flex items-center justify-between"
        >
            <div
                class="flex overflow-hidden bg-white border divide-x rounded-lg rtl:flex-row-reverse dark:bg-gray-900 dark:border-gray-700 dark:divide-gray-700"
            >
                <h1
                    class="px-4 py-2 font-medium text-gray-600 sm:px-6 dark:text-gray-300"
                >
                    {document_name}
                </h1>
            </div>

            <div
                class="flex overflow-hidden bg-white border divide-x rounded-lg rtl:flex-row-reverse dark:bg-gray-900 dark:border-gray-700 dark:divide-gray-700"
            >
                <button
                    title="Save"
                    onclick={save}
                    class="px-4 py-2 font-medium text-gray-600 transition-colors duration-200 sm:px-6 dark:hover:bg-gray-800 dark:text-gray-300 hover:bg-gray-100"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 640 640"
                        class="w-5 h-5 sm:w-6 sm:h-6"
                        stroke="currentColor"
                        fill="currentColor"
                        stroke-width="1.5"
                        ><path
                            d="M160 144C151.2 144 144 151.2 144 160L144 480C144 488.8 151.2 496 160 496L480 496C488.8 496 496 488.8 496 480L496 237.3C496 233.1 494.3 229 491.3 226L416 150.6L416 240C416 257.7 401.7 272 384 272L224 272C206.3 272 192 257.7 192 240L192 144L160 144zM240 144L240 224L368 224L368 144L240 144zM96 160C96 124.7 124.7 96 160 96L402.7 96C419.7 96 436 102.7 448 114.7L525.3 192C537.3 204 544 220.3 544 237.3L544 480C544 515.3 515.3 544 480 544L160 544C124.7 544 96 515.3 96 480L96 160zM256 384C256 348.7 284.7 320 320 320C355.3 320 384 348.7 384 384C384 419.3 355.3 448 320 448C284.7 448 256 419.3 256 384z"
                        /></svg
                    >
                </button>

                <button
                    title="Open"
                    onclick={open}
                    class="px-4 py-2 font-medium text-gray-600 transition-colors duration-200 sm:px-6 dark:hover:bg-gray-800 dark:text-gray-300 hover:bg-gray-100"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 640 640"
                        class="w-5 h-5 sm:w-6 sm:h-6"
                        stroke="currentColor"
                        fill="currentColor"
                        stroke-width="1.5"
                    >
                        <path
                            d="M304 112L192 112C183.2 112 176 119.2 176 128L176 512C176 520.8 183.2 528 192 528L448 528C456.8 528 464 520.8 464 512L464 272L376 272C336.2 272 304 239.8 304 200L304 112zM444.1 224L352 131.9L352 200C352 213.3 362.7 224 376 224L444.1 224zM128 128C128 92.7 156.7 64 192 64L325.5 64C342.5 64 358.8 70.7 370.8 82.7L493.3 205.3C505.3 217.3 512 233.6 512 250.6L512 512C512 547.3 483.3 576 448 576L192 576C156.7 576 128 547.3 128 512L128 128z"
                        />
                    </svg>
                </button>
            </div>
        </header>

        <textarea
            class="min-h-0 flex-1 resize-none bg-zinc-950 p-4 font-mono text-base leading-6 text-zinc-100 outline-none placeholder:text-zinc-600"
            bind:value={documentText}
            onclick={updateCursor}
            onkeyup={updateCursor}
            onselect={updateCursor}
            oninput={(e) => {
                updateCursor(e);
                saved_state = "not saved";
            }}
            placeholder="Start typing..."
            spellcheck="false"></textarea>

        <footer class="px-4 py-3 flex items-center justify-between">
            <p class="text-xs text-zinc-500">{saved_state}</p>
            <p class="text-xs text-zinc-500">
                Characters: {documentText.length} | Cursor: {cursorPosition}
            </p>
        </footer>
    </section>
</main>
