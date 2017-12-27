function instantiateTemplate(tplElt, placeHolders) {
    placeHolders = placeHolders || {};

    for (const selector in placeHolders) {
        if (!placeHolders.hasOwnProperty(selector)) {
            continue;
        }

        tplElt.content.querySelector(selector).textContent = placeHolders[selector];
    }

    return document.importNode(tplElt.content, true);
}

export default {
    id: document.getElementById.bind(document),
    instanciate: instantiateTemplate,
};
