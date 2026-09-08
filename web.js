const element = (id) => document.getElementById(id);
const fields = ["tokens", "ast", "value"];
element("example").addEventListener("change", (event) => {
  element("source").value = event.target.value;
});

try {
  // import自体の失敗も画面に表示します。
  const { default: init, run } = await import("./pkg/tiny_exp.js");
  await init();
  element("status").textContent = "準備できました。式を入力して実行してください。";
  element("execute").disabled = false;
  element("runner").addEventListener("submit", (event) => {
    event.preventDefault();
    fields.forEach((field) => { element(field).textContent = "未完了"; });
    element("error").textContent = "";
    let output;
    try {
      output = run(element("source").value);
      fields.forEach((field) => {
        element(field).textContent = output[field] || "未完了";
      });
      element("error").textContent = output.error;
      element("status").textContent = output.error ? "処理が途中で止まりました。" : "評価が完了しました。";
    } catch (error) {
      element("error").textContent = `実行に失敗しました: ${error}`;
      element("status").textContent = "実行エラー";
    } finally {
      output?.free();
    }
  });
} catch (error) {
  element("status").textContent = "Wasmの読み込みに失敗しました。READMEのビルド・起動手順を確認してください。";
  element("error").textContent = String(error);
}
