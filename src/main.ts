import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import { writeTextFile } from "@tauri-apps/plugin-fs";

let urlInputEl: HTMLInputElement | null;
let statusMsgEl: HTMLElement | null;
let outputContainerEl: HTMLElement | null;
let outputContentEl: HTMLElement | null;
let extractButtonEl: HTMLButtonElement | null;
let copyButtonEl: HTMLButtonElement | null;
let saveButtonEl: HTMLButtonElement | null;

let currentMarkdown = "";

async function extractStyles() {
  if (!urlInputEl || !statusMsgEl || !outputContainerEl || !outputContentEl || !extractButtonEl) {
    return;
  }

  const url = urlInputEl.value.trim();
  
  if (!url) {
    showStatus("Please enter a valid URL", "error");
    return;
  }

  // Show loading state
  extractButtonEl.disabled = true;
  extractButtonEl.innerHTML = '<span class="spinner"></span><span class="button-text">Extracting...</span>';
  showStatus("Fetching website and extracting styles...", "loading");
  outputContainerEl.classList.add("hidden");

  try {
    // Call Rust backend to extract styles
    const markdown: string = await invoke("extract_website_styles", { url });
    
    currentMarkdown = markdown;
    outputContentEl.textContent = markdown;
    outputContainerEl.classList.remove("hidden");
    
    showStatus("✅ Style guide generated successfully!", "success");
  } catch (error) {
    console.error("Error extracting styles:", error);
    showStatus(`❌ Error: ${error}`, "error");
  } finally {
    extractButtonEl.disabled = false;
    extractButtonEl.innerHTML = '<span class="button-text">Extract Styles</span>';
  }
}

function showStatus(message: string, type: "loading" | "success" | "error") {
  if (!statusMsgEl) return;
  
  statusMsgEl.textContent = message;
  statusMsgEl.className = `status-msg ${type}`;
  
  // Auto-hide success messages after 3 seconds
  if (type === "success") {
    setTimeout(() => {
      if (statusMsgEl && statusMsgEl.classList.contains("success")) {
        statusMsgEl.textContent = "";
        statusMsgEl.className = "status-msg";
      }
    }, 3000);
  }
}

async function copyToClipboard() {
  if (!currentMarkdown) return;
  
  try {
    await navigator.clipboard.writeText(currentMarkdown);
    showStatus("✅ Markdown copied to clipboard!", "success");
  } catch (error) {
    console.error("Error copying to clipboard:", error);
    showStatus("❌ Failed to copy to clipboard", "error");
  }
}

async function saveToFile() {
  if (!currentMarkdown) return;
  
  try {
    const filePath = await save({
      defaultPath: "style-guide.md",
      filters: [{
        name: "Markdown",
        extensions: ["md"]
      }]
    });
    
    if (filePath) {
      await writeTextFile(filePath, currentMarkdown);
      showStatus("✅ File saved successfully!", "success");
    }
  } catch (error) {
    console.error("Error saving file:", error);
    showStatus("❌ Failed to save file", "error");
  }
}

window.addEventListener("DOMContentLoaded", () => {
  urlInputEl = document.querySelector("#url-input");
  statusMsgEl = document.querySelector("#status-msg");
  outputContainerEl = document.querySelector("#output-container");
  outputContentEl = document.querySelector("#output-content");
  extractButtonEl = document.querySelector("#extract-button");
  copyButtonEl = document.querySelector("#copy-button");
  saveButtonEl = document.querySelector("#save-button");
  
  document.querySelector("#extract-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    extractStyles();
  });
  
  copyButtonEl?.addEventListener("click", copyToClipboard);
  saveButtonEl?.addEventListener("click", saveToFile);
});
