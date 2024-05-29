// there will be a div with the id of "consituencies" in the index.html file
// each subdiv will have a data attribute for the following keys:
// - name
// - labour-probability
// - conservative-probability
// - other-probability
// - favourite-lead
// We want to sort the consituencies by the key passed in the function

const sorters = document.querySelectorAll("[data-sort]");

for (const sorter of sorters) {
  sorter.addEventListener("click", (e) => {
    const key = e.target.dataset.sort;
    const asc = e.target.dataset.asc === "true";
    const sortIsNumeric = e.target.dataset.numeric === "true";

    sortConsituenciesBy(key, sortIsNumeric, asc);

    // remove the asc attribute from all the sorters
    for (const sorter of sorters) {
      sorter.removeAttribute("data-asc");
    }

    // set the asc attribute to the opposite of the current one
    e.target.dataset.asc = !asc;
  });
}

const sortConsituenciesBy = (key, sortIsNumeric, asc) => {
  const consituenciesDiv = document.getElementById("constituencies");

  const constituenciesArray = Array.from(consituenciesDiv.children);

  if (asc) {
    constituenciesArray.sort((a, b) => {
      let aData = a.dataset[key];
      let bData = b.dataset[key];
      if (sortIsNumeric) {
        aData = parseFloat(aData);
        bData = parseFloat(bData);
      }
      return aData - bData;
    });
  } else {
    constituenciesArray.sort((a, b) => {
      let aData = a.dataset[key];
      let bData = b.dataset[key];
      if (sortIsNumeric) {
        aData = parseFloat(aData);
        bData = parseFloat(bData);
      }
      return bData - aData;
    });
  }

  // first remove all the children
  // then add the sorted children

  consituenciesDiv.innerHTML = "";

  constituenciesArray.forEach((constituency) => {
    consituenciesDiv.appendChild(constituency);
  });
};
