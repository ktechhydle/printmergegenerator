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
