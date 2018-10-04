function start(jbob: typeof import('./jbob')) {
    console.log("Starting interpreter")

    // Create a new context, and load our sample source.
    const ctx = jbob.JBobContext.new();
    ctx.requireJBob();
    ctx.requireLittleProver();

    const output = document.getElementById("output");
    const input = document.getElementById("input");
    if (output == null || !(input instanceof HTMLTextAreaElement)) {
        throw "couldn't find input and output elements";
    }

    const appendOutput = function (klass: string, text: string) {
        const p = document.createElement("p");
        p.setAttribute("class", klass);
        p.innerText = text;
        output.appendChild(p);
    }

    input.addEventListener("keypress", function (e) {
        // Did the user press enter?
        if (e.which == 13) {
            // Did the user press enter with the cursor after a
            // single valid s-expr? If not, pass the key through.
            if (input.selectionStart != input.selectionEnd) {
                return true;
            }
            const selection = input.selectionStart;
            console.log("selection:", selection);
            const rawSource = input.value;
            const beforeCursor = rawSource.substring(0, selection);
            const afterCursor = rawSource.substring(selection);
            if (!afterCursor.match(/^[ \t\r\n]*$/) || !ctx.isValidSExpr(beforeCursor)) {
                //console.log("beforeCursor:", beforeCursor);
                return true;
            }

            // Get the source code and send it to our interpreter.
            e.preventDefault();
            const source = rawSource.replace(/[ \t\r\n]+$/, '');
            console.log("Evalulating", source);
            appendOutput("input", source)
            const value = ctx.eval(source);
            console.log("Evaluated");
            try {
                appendOutput("output", value.toString());
            } finally {
                value.free();
            }
            return false;
        }
        return true;
    });
}

async function load() {
    start(await import('./jbob'));
}

load()

