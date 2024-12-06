let totalPersons = 2;
const counts = {
    adults: 2,
    teenagers: 0,
    children: 0,
    toddlers: 0,
};

function toggleDropdown() {
    const dropdownContent = document.getElementById('dropdown-content');
    dropdownContent.style.display = dropdownContent.style.display === 'block' ? 'none' : 'block';
}

function toggleDropdownTypeVerblijf() {
    const dropdownContent = document.getElementById('dropdown-content-type-verblijf');
    dropdownContent.style.display = dropdownContent.style.display === 'block' ? 'none' : 'block';
}

function SetVerblijf(verblijf) {
    document.getElementById("type-verblijf").textContent = verblijf;
}

function increaseCount(category) {
    counts[category]++;
    updateCounts();
}

function decreaseCount(category) {
    if (counts[category] > 0) {
        counts[category]--;
        updateCounts();
    }
}

function updateCounts() {
    document.getElementById('adults').textContent = counts.adults;
    document.getElementById('teenagers').textContent = counts.teenagers;
    document.getElementById('children').textContent = counts.children;
    document.getElementById('toddlers').textContent = counts.toddlers;

    totalPersons = counts.adults + counts.teenagers + counts.children + counts.toddlers;
    document.getElementById('total-persons').textContent = totalPersons;
}