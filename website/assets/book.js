// Ovie Book & Module System Interactive Features

document.addEventListener('DOMContentLoaded', function () {
  initModuleTabs();
  initBookInteractions();
  initDownloadTracking();
});

// Module demo tabs
function initModuleTabs() {
  const tabs = document.querySelectorAll('.mdtab');
  tabs.forEach(tab => {
    tab.addEventListener('click', function () {
      const target = this.getAttribute('data-tab');
      // Deactivate all
      document.querySelectorAll('.mdtab').forEach(t => t.classList.remove('active'));
      document.querySelectorAll('.mdcontent').forEach(c => c.classList.remove('active'));
      // Activate selected
      this.classList.add('active');
      const content = document.getElementById('md-' + target);
      if (content) content.classList.add('active');
    });
  });
}

// Book 3D hover and chapter interactions
function initBookInteractions() {
  const book = document.querySelector('.book-3d');
  if (!book) return;

  // Enhanced 3D tilt on mouse move
  const bookSection = document.querySelector('.book-showcase');
  if (bookSection) {
    bookSection.addEventListener('mousemove', function (e) {
      const rect = book.getBoundingClientRect();
      const centerX = rect.left + rect.width / 2;
      const centerY = rect.top + rect.height / 2;
      const deltaX = (e.clientX - centerX) / 20;
      const deltaY = (e.clientY - centerY) / 20;
      book.style.transform = `perspective(1000px) rotateY(${-20 + deltaX}deg) rotateX(${-deltaY}deg)`;
    });

    bookSection.addEventListener('mouseleave', function () {
      book.style.transform = 'perspective(1000px) rotateY(-20deg)';
    });
  }

  // Chapter items — highlight on hover
  const chapterItems = document.querySelectorAll('.chapter-item');
  chapterItems.forEach((item, index) => {
    item.style.animationDelay = `${index * 0.05}s`;
    item.addEventListener('mouseenter', function () {
      this.style.background = 'rgba(245, 158, 11, 0.08)';
    });
    item.addEventListener('mouseleave', function () {
      this.style.background = '';
    });
  });
}

// Download button feedback
function initDownloadTracking() {
  const downloadBtns = document.querySelectorAll('.download-btns .btn, .download-source .btn');
  downloadBtns.forEach(btn => {
    btn.addEventListener('click', function () {
      const original = this.innerHTML;
      this.innerHTML = '<span class="btn-icon">✅</span> Starting download...';
      setTimeout(() => { this.innerHTML = original; }, 2500);
    });
  });

  // Copy install commands on click
  const installCmds = document.querySelectorAll('.install-cmd code');
  installCmds.forEach(cmd => {
    cmd.style.cursor = 'pointer';
    cmd.title = 'Click to copy';
    cmd.addEventListener('click', function () {
      navigator.clipboard.writeText(this.textContent).then(() => {
        const original = this.textContent;
        this.textContent = '✅ Copied!';
        setTimeout(() => { this.textContent = original; }, 1500);
      }).catch(() => {});
    });
  });
}
