const form = document.getElementById("controls");
const errorDialog = document.getElementById("error-dialog");

function showErrorDialog(msg) {
  errorDialog.showModal();

  let errorMsg = document.getElementById("error-dialog-msg");
  errorMsg.innerText = msg;
}

function closeErrorDialog() {
  errorDialog.close();
}

async function createOutput() {
  const formData = new FormData(form);
  const start = formData.get("start");
  const end = formData.get("end");
  const repeat = formData.get("repeat");
  const prefix = formData.get("prefix");
  const vertical = document.getElementById("count-numbers-vertically").checked;
  const numbered = document.getElementById("number-spots").checked;

  console.log(startRange);
}

function downloadOutput() {
  const formData = new FormData(form);
  const filename = formData.get("filename");

  const blob = new Blob([filename], { type: "text/plain" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

window.closeErrorDialog = closeErrorDialog;
window.showErrorDialog = showErrorDialog;
window.downloadOutput = downloadOutput;

// change the output preview everytime a form value is modified
form.addEventListener("input", createOutput);
