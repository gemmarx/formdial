

onload = function() {
    const prefix = 'formdial';
    const submit = 'formdial-button-submit';

    document.getElementById('content').innerHTML = 
        markdownit({html: true})
        .use(input, {prefix: prefix})
        .render(document.getElementById('content').innerHTML);

    document.getElementById(submit).onclick = function() {
        let inputs = document.getElementsByTagName('input');
        let areas = document.getElementsByTagName('textarea');
        let selects = document.getElementsByTagName('select');
        let result = {};
        result = take_inputs(result, inputs);
        if (null === result) return;
        for(var i=0; i<areas.length; i++) {
            let v = areas[i];
            if (false === v.validity.valid) return;
            result[v.name] = v.value;
        }
        for(var i=0; i<selects.length; i++) {
            let v = selects[i];
            result[v.name] = v.value;
        }
        external.invoke(JSON.stringify(result));
    }
};

function take_inputs(result, inputs) {
    for(var i=0; i<inputs.length; i++) {
        let v = inputs[i];
        let t = v.type;
        if (false === v.validity.valid) return;
        if ('radio' === t) {
            let id = v.name;
            if (!result[id]) {
                result[id] = v.checked ? v.value : "";
            }
        } else if ('checkbox' === t) {
            let id = v.name + '-' + v.value;
            result[id] = v.checked;
        } else {
            let id = v.name;
            result[id] = v.value;
        }
    }
    return result;
}

