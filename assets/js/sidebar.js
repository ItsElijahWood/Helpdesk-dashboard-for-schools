const sidebar_light = '/assets/imgs/sidebar-light.svg';
const sidebar_open_light = '/assets/imgs/sidebar-open-light.svg';

const sidebar = document.getElementById('sidebar');
const sidebar_nav = document.getElementById("sidebar-nav");
const btn = document.getElementById('sidebarButton');
const questionText = document.getElementById('sidebarQuestionText');

let open = false;

function setOpen(nextOpen) {
  open = nextOpen;

  btn.src = open ? sidebar_open_light : sidebar_light;

  sidebar.classList.toggle('w-64', open);

  sidebar_nav.classList.toggle('pointer-events-none', !open);

  questionText.classList.toggle('hidden', !open);
}

document.addEventListener('click', (e) => {
  const clickedBtn = btn.contains(e.target);
  const clickedSidebar = sidebar.contains(e.target);

  if (clickedBtn) {
    setOpen(!open);
    return;
  }

  if (!open && clickedSidebar) {
      setOpen(true);
      return;
    }

  if (open && !clickedSidebar) {
    setOpen(false);
  }
});

setOpen(false);
