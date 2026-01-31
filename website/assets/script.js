// Ovie Programming Language Website - Interactive Features
// iOS 26 Mobile-First Design with Modern ES6+ Features

document.addEventListener('DOMContentLoaded', function() {
    // Initialize all interactive features
    initializeNavigation();
    initializeMobileMenu();
    initializeTabs();
    initializePlayground();
    initializeAnimations();
    initializeCodeHighlighting();
    initializeExamples();
    initializeTouchInteractions();
    initializeDownloads();
});

// Navigation functionality with iOS 26 enhancements
function initializeNavigation() {
    const navbar = document.querySelector('.navbar');
    const navLinks = document.querySelectorAll('.nav-link');
    
    // Smooth scrolling for navigation links
    navLinks.forEach(link => {
        link.addEventListener('click', function(e) {
            const href = this.getAttribute('href');
            
            // Handle external links
            if (href.startsWith('http') || href.includes('.html')) {
                return; // Let the browser handle it normally
            }
            
            // Handle anchor links
            if (href.startsWith('#')) {
                e.preventDefault();
                const targetSection = document.querySelector(href);
                
                if (targetSection) {
                    targetSection.scrollIntoView({
                        behavior: 'smooth',
                        block: 'start'
                    });
                    
                    // Close mobile menu if open
                    const mobileMenu = document.querySelector('.mobile-menu');
                    if (mobileMenu && mobileMenu.classList.contains('active')) {
                        mobileMenu.classList.remove('active');
                        document.body.classList.remove('menu-open');
                    }
                }
            }
        });
    });
    
    // Enhanced navbar background on scroll with iOS 26 blur effect
    let ticking = false;
    
    function updateNavbar() {
        if (window.scrollY > 50) {
            navbar.style.background = 'rgba(13, 17, 23, 0.9)';
            navbar.style.backdropFilter = 'blur(20px)';
        } else {
            navbar.style.background = 'rgba(13, 17, 23, 0.8)';
            navbar.style.backdropFilter = 'blur(20px)';
        }
        ticking = false;
    }
    
    window.addEventListener('scroll', function() {
        if (!ticking) {
            requestAnimationFrame(updateNavbar);
            ticking = true;
        }
    });
    
    // Active navigation highlighting
    const sections = document.querySelectorAll('section[id]');
    const observerOptions = {
        threshold: 0.3,
        rootMargin: '-100px 0px -100px 0px'
    };
    
    const observer = new IntersectionObserver(function(entries) {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                const activeLink = document.querySelector(`.nav-link[href="#${entry.target.id}"]`);
                navLinks.forEach(link => link.classList.remove('active'));
                if (activeLink) {
                    activeLink.classList.add('active');
                }
            }
        });
    }, observerOptions);
    
    sections.forEach(section => observer.observe(section));
}

// Mobile menu functionality with iOS 26 animations
function initializeMobileMenu() {
    const mobileToggle = document.querySelector('.mobile-menu-toggle');
    const mobileMenu = document.querySelector('.mobile-menu');
    const mobileLinks = document.querySelectorAll('.mobile-nav-link');
    
    if (!mobileToggle || !mobileMenu) {
        // Create mobile menu elements if they don't exist
        createMobileMenu();
        return;
    }
    
    // Toggle mobile menu
    mobileToggle.addEventListener('click', function(e) {
        e.preventDefault();
        e.stopPropagation();
        
        const isActive = mobileMenu.classList.contains('active');
        
        if (isActive) {
            closeMobileMenu();
        } else {
            openMobileMenu();
        }
    });
    
    // Close menu when clicking on links
    mobileLinks.forEach(link => {
        link.addEventListener('click', function() {
            // Add small delay for smooth transition
            setTimeout(() => {
                closeMobileMenu();
            }, 150);
        });
    });
    
    // Close menu when clicking outside
    document.addEventListener('click', function(e) {
        if (mobileMenu && mobileMenu.classList.contains('active') && 
            !mobileMenu.contains(e.target) && !mobileToggle.contains(e.target)) {
            closeMobileMenu();
        }
    });
    
    // Handle escape key
    document.addEventListener('keydown', function(e) {
        if (e.key === 'Escape' && mobileMenu && mobileMenu.classList.contains('active')) {
            closeMobileMenu();
        }
    });
    
    // Handle window resize
    window.addEventListener('resize', function() {
        if (window.innerWidth > 768 && mobileMenu && mobileMenu.classList.contains('active')) {
            closeMobileMenu();
        }
    });
    
    function openMobileMenu() {
        mobileMenu.classList.add('active');
        mobileToggle.innerHTML = '✕';
        mobileToggle.setAttribute('aria-label', 'Close mobile menu');
        document.body.classList.add('menu-open');
        
        // Prevent body scroll
        document.body.style.overflow = 'hidden';
        
        // Add iOS 26 haptic feedback simulation
        if (navigator.vibrate) {
            navigator.vibrate(10);
        }
    }
    
    function closeMobileMenu() {
        mobileMenu.classList.remove('active');
        mobileToggle.innerHTML = '☰';
        mobileToggle.setAttribute('aria-label', 'Open mobile menu');
        document.body.classList.remove('menu-open');
        
        // Restore body scroll
        document.body.style.overflow = '';
        
        // Add iOS 26 haptic feedback simulation
        if (navigator.vibrate) {
            navigator.vibrate(5);
        }
    }
}

function createMobileMenu() {
    const navbar = document.querySelector('.navbar');
    const navContainer = document.querySelector('.nav-container');
    const navMenu = document.querySelector('.nav-menu');
    const navActions = document.querySelector('.nav-actions');
    
    if (!navbar || !navContainer) return;
    
    // Create mobile toggle button
    const mobileToggle = document.createElement('button');
    mobileToggle.className = 'mobile-menu-toggle';
    mobileToggle.innerHTML = '☰';
    mobileToggle.setAttribute('aria-label', 'Open mobile menu');
    mobileToggle.setAttribute('type', 'button');
    
    // Add toggle to nav actions or create nav actions if it doesn't exist
    if (navActions) {
        // Remove theme toggle if it exists
        const themeToggle = navActions.querySelector('#themeToggle, .theme-toggle');
        if (themeToggle) {
            themeToggle.remove();
        }
        navActions.appendChild(mobileToggle);
    } else {
        const newNavActions = document.createElement('div');
        newNavActions.className = 'nav-actions';
        newNavActions.appendChild(mobileToggle);
        navContainer.appendChild(newNavActions);
    }
    
    // Create mobile menu
    const mobileMenu = document.createElement('div');
    mobileMenu.className = 'mobile-menu';
    
    const mobileNavLinks = document.createElement('div');
    mobileNavLinks.className = 'mobile-nav-links';
    
    // Copy navigation links to mobile menu
    if (navMenu) {
        const links = navMenu.querySelectorAll('.nav-link');
        links.forEach(link => {
            const mobileLink = document.createElement('a');
            mobileLink.href = link.href;
            mobileLink.textContent = link.textContent;
            mobileLink.className = 'mobile-nav-link';
            mobileNavLinks.appendChild(mobileLink);
        });
    }
    
    // Add default navigation if no nav menu exists
    if (!navMenu || navMenu.children.length === 0) {
        const defaultLinks = [
            { href: '#home', text: 'Home' },
            { href: '#features', text: 'Features' },
            { href: '#download', text: 'Download' },
            { href: '#docs', text: 'Documentation' },
            { href: '#examples', text: 'Examples' },
            { href: '#library', text: 'Library' },
            { href: '#community', text: 'Community' }
        ];
        
        defaultLinks.forEach(linkData => {
            const mobileLink = document.createElement('a');
            mobileLink.href = linkData.href;
            mobileLink.textContent = linkData.text;
            mobileLink.className = 'mobile-nav-link';
            mobileNavLinks.appendChild(mobileLink);
        });
    }
    
    // Create mobile actions
    const mobileNavActions = document.createElement('div');
    mobileNavActions.className = 'mobile-nav-actions';
    
    const getStartedBtn = document.createElement('a');
    getStartedBtn.href = '#download';
    getStartedBtn.className = 'btn btn-primary';
    getStartedBtn.innerHTML = '<span class="btn-icon">⬇️</span> Get Started';
    
    const githubBtn = document.createElement('a');
    githubBtn.href = 'https://github.com/southwarridev/ovie';
    githubBtn.className = 'btn btn-secondary';
    githubBtn.innerHTML = '<span class="btn-icon">📱</span> GitHub';
    githubBtn.target = '_blank';
    githubBtn.rel = 'noopener noreferrer';
    
    const gitlabBtn = document.createElement('a');
    gitlabBtn.href = 'https://gitlab.com/ovie1/ovie';
    gitlabBtn.className = 'btn btn-secondary';
    gitlabBtn.innerHTML = '<span class="btn-icon">🦊</span> GitLab';
    gitlabBtn.target = '_blank';
    gitlabBtn.rel = 'noopener noreferrer';
    
    mobileNavActions.appendChild(getStartedBtn);
    mobileNavActions.appendChild(githubBtn);
    mobileNavActions.appendChild(gitlabBtn);
    
    mobileMenu.appendChild(mobileNavLinks);
    mobileMenu.appendChild(mobileNavActions);
    
    // Insert mobile menu after navbar
    navbar.parentNode.insertBefore(mobileMenu, navbar.nextSibling);
    
    // Initialize mobile menu functionality
    initializeMobileMenu();
}

// Tab functionality
function initializeTabs() {
    const tabContainers = document.querySelectorAll('[data-tab]');
    
    tabContainers.forEach(container => {
        const tabButtons = container.parentElement.querySelectorAll('.tab-btn');
        const tabContents = container.parentElement.querySelectorAll('.tab-content');
        
        tabButtons.forEach(button => {
            button.addEventListener('click', function() {
                const targetTab = this.getAttribute('data-tab');
                
                // Remove active class from all buttons and contents
                tabButtons.forEach(btn => btn.classList.remove('active'));
                tabContents.forEach(content => content.classList.remove('active'));
                
                // Add active class to clicked button and corresponding content
                this.classList.add('active');
                const targetContent = document.getElementById(targetTab);
                if (targetContent) {
                    targetContent.classList.add('active');
                }
            });
        });
    });
    
    // Initialize download tabs
    const downloadTabs = document.querySelectorAll('.code-tabs .tab-btn');
    downloadTabs.forEach(button => {
        button.addEventListener('click', function() {
            const targetTab = this.getAttribute('data-tab');
            const tabContainer = this.closest('.code-block');
            const tabButtons = tabContainer.querySelectorAll('.tab-btn');
            const tabContents = tabContainer.querySelectorAll('.tab-content');
            
            tabButtons.forEach(btn => btn.classList.remove('active'));
            tabContents.forEach(content => content.classList.remove('active'));
            
            this.classList.add('active');
            const targetContent = tabContainer.querySelector(`#${targetTab}`);
            if (targetContent) {
                targetContent.classList.add('active');
            }
        });
    });
    
    // Initialize examples tabs
    const exampleTabs = document.querySelectorAll('.examples-tabs .tab-btn');
    exampleTabs.forEach(button => {
        button.addEventListener('click', function() {
            const targetTab = this.getAttribute('data-tab');
            
            exampleTabs.forEach(btn => btn.classList.remove('active'));
            document.querySelectorAll('.examples-content .tab-content').forEach(content => {
                content.classList.remove('active');
            });
            
            this.classList.add('active');
            const targetContent = document.getElementById(targetTab);
            if (targetContent) {
                targetContent.classList.add('active');
            }
        });
    });
}

// Interactive playground
function initializePlayground() {
    const runButton = document.getElementById('runCode');
    const shareButton = document.getElementById('shareCode');
    const codeEditor = document.getElementById('codeEditor');
    const outputConsole = document.getElementById('outputConsole');
    const playgroundTabs = document.querySelectorAll('.playground-tabs .tab-btn');
    const exampleButtons = document.querySelectorAll('.example-btn');
    
    // Example code snippets
    const examples = {
        hello: `// Hello World in Ovie
seeAm "Hello, World!"
seeAm "Welcome to Ovie programming!"`,
        
        calculator: `// Simple Calculator in Ovie
fn add(a, b) {
    return a + b
}

fn multiply(a, b) {
    return a * b
}

mut x = 10
mut y = 5

seeAm "Addition: " + add(x, y)
seeAm "Multiplication: " + multiply(x, y)`,
        
        fibonacci: `// Fibonacci Sequence in Ovie
fn fibonacci(n) {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

seeAm "Fibonacci sequence:"
for i in 0..10 {
    seeAm "F(" + i + ") = " + fibonacci(i)
}`,
        
        struct: `// Data Structures in Ovie
struct Person {
    name: String,
    age: Number,
    email: String,
}

struct Company {
    name: String,
    employees: [Person],
}

mut person = Person {
    name: "Alice Johnson",
    age: 30,
    email: "alice@example.com",
}

mut company = Company {
    name: "Tech Corp",
    employees: [person],
}

seeAm "Company: " + company.name
seeAm "Employee: " + person.name + " (" + person.age + " years old)"`
    };
    
    // Playground tab switching
    playgroundTabs.forEach(button => {
        button.addEventListener('click', function() {
            const targetTab = this.getAttribute('data-tab');
            
            playgroundTabs.forEach(btn => btn.classList.remove('active'));
            document.querySelectorAll('.playground-content .tab-content').forEach(content => {
                content.classList.remove('active');
            });
            
            this.classList.add('active');
            const targetContent = document.getElementById(targetTab);
            if (targetContent) {
                targetContent.classList.add('active');
            }
        });
    });
    
    // Example button functionality
    exampleButtons.forEach(button => {
        button.addEventListener('click', function() {
            const exampleKey = this.getAttribute('data-example');
            if (examples[exampleKey]) {
                codeEditor.value = examples[exampleKey];
                
                // Switch to editor tab
                playgroundTabs.forEach(btn => btn.classList.remove('active'));
                document.querySelectorAll('.playground-content .tab-content').forEach(content => {
                    content.classList.remove('active');
                });
                
                document.querySelector('.playground-tabs .tab-btn[data-tab="editor"]').classList.add('active');
                document.getElementById('editor').classList.add('active');
            }
        });
    });
    
    // Run code functionality
    runButton.addEventListener('click', function() {
        const code = codeEditor.value.trim();
        if (!code) {
            showOutput('Error: No code to run', 'error');
            return;
        }
        
        // Simulate code execution
        runButton.textContent = 'Running...';
        runButton.disabled = true;
        
        setTimeout(() => {
            simulateOvieExecution(code);
            runButton.textContent = 'Run Code';
            runButton.disabled = false;
        }, 1000);
    });
    
    // Share code functionality
    shareButton.addEventListener('click', function() {
        const code = codeEditor.value.trim();
        if (!code) {
            showOutput('Error: No code to share', 'error');
            return;
        }
        
        // Create shareable URL (simulated)
        const encodedCode = btoa(encodeURIComponent(code));
        const shareUrl = `${window.location.origin}${window.location.pathname}?code=${encodedCode}`;
        
        // Copy to clipboard
        navigator.clipboard.writeText(shareUrl).then(() => {
            showOutput('✅ Share URL copied to clipboard!', 'success');
        }).catch(() => {
            showOutput('Share URL: ' + shareUrl, 'info');
        });
    });
    
    // Load shared code from URL
    const urlParams = new URLSearchParams(window.location.search);
    const sharedCode = urlParams.get('code');
    if (sharedCode) {
        try {
            const decodedCode = decodeURIComponent(atob(sharedCode));
            codeEditor.value = decodedCode;
        } catch (e) {
            console.error('Failed to load shared code:', e);
        }
    }
    
    function simulateOvieExecution(code) {
        // Switch to output tab
        playgroundTabs.forEach(btn => btn.classList.remove('active'));
        document.querySelectorAll('.playground-content .tab-content').forEach(content => {
            content.classList.remove('active');
        });
        
        document.querySelector('.playground-tabs .tab-btn[data-tab="output"]').classList.add('active');
        document.getElementById('output').classList.add('active');
        
        // Clear previous output
        outputConsole.innerHTML = '';
        
        // Simulate compilation
        showOutput('🔄 Compiling with Ovie self-hosted compiler...', 'info');
        
        setTimeout(() => {
            showOutput('✅ Compilation successful!', 'success');
            showOutput('🚀 Running program...', 'info');
            
            setTimeout(() => {
                // Parse and simulate seeAm statements
                const lines = code.split('\n');
                let hasOutput = false;
                
                lines.forEach(line => {
                    const trimmed = line.trim();
                    if (trimmed.startsWith('seeAm ')) {
                        const match = trimmed.match(/seeAm\s+"([^"]+)"/);
                        if (match) {
                            showOutput(match[1], 'output');
                            hasOutput = true;
                        }
                    }
                });
                
                // Simulate other outputs based on code content
                if (code.includes('fibonacci')) {
                    for (let i = 0; i < 10; i++) {
                        const fib = fibonacci(i);
                        showOutput(`F(${i}) = ${fib}`, 'output');
                    }
                    hasOutput = true;
                }
                
                if (code.includes('add(') && code.includes('multiply(')) {
                    showOutput('Addition: 15', 'output');
                    showOutput('Multiplication: 50', 'output');
                    hasOutput = true;
                }
                
                if (code.includes('struct Person')) {
                    showOutput('Company: Tech Corp', 'output');
                    showOutput('Employee: Alice Johnson (30 years old)', 'output');
                    hasOutput = true;
                }
                
                if (!hasOutput) {
                    showOutput('Program executed successfully (no output)', 'success');
                }
                
                showOutput('✅ Program completed', 'success');
            }, 500);
        }, 800);
    }
    
    function showOutput(message, type = 'output') {
        const line = document.createElement('div');
        line.className = `console-line ${type}`;
        line.textContent = message;
        outputConsole.appendChild(line);
        outputConsole.scrollTop = outputConsole.scrollHeight;
    }
    
    function fibonacci(n) {
        if (n <= 1) return n;
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

// Animation on scroll
function initializeAnimations() {
    const animatedElements = document.querySelectorAll('.feature-card, .example-card, .library-module, .community-card');
    
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };
    
    const observer = new IntersectionObserver(function(entries) {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('fade-in-up');
            }
        });
    }, observerOptions);
    
    animatedElements.forEach(element => {
        observer.observe(element);
    });
}

// Code syntax highlighting
function initializeCodeHighlighting() {
    const codeBlocks = document.querySelectorAll('code.language-ovie');
    
    codeBlocks.forEach(block => {
        highlightOvieCode(block);
    });
    
    function highlightOvieCode(element) {
        let code = element.textContent;
        
        // Keywords
        const keywords = ['fn', 'mut', 'if', 'else', 'for', 'in', 'return', 'struct', 'test', 'seeAm'];
        keywords.forEach(keyword => {
            const regex = new RegExp(`\\b${keyword}\\b`, 'g');
            code = code.replace(regex, `<span class="keyword">${keyword}</span>`);
        });
        
        // Strings
        code = code.replace(/"([^"]+)"/g, '<span class="string">"$1"</span>');
        
        // Numbers
        code = code.replace(/\b\d+\b/g, '<span class="number">$&</span>');
        
        // Comments
        code = code.replace(/\/\/.*$/gm, '<span class="comment">$&</span>');
        
        // Functions
        code = code.replace(/\b(\w+)\s*\(/g, '<span class="function">$1</span>(');
        
        element.innerHTML = code;
    }
}

// Example interactions
function initializeExamples() {
    const exampleLinks = document.querySelectorAll('.example-link');
    
    exampleLinks.forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            
            // Show modal or navigate to example page
            const exampleName = this.textContent;
            showExampleModal(exampleName, this.href);
        });
    });
    
    function showExampleModal(name, url) {
        // Create modal overlay
        const modal = document.createElement('div');
        modal.className = 'example-modal';
        modal.innerHTML = `
            <div class="modal-content">
                <div class="modal-header">
                    <h3>${name}</h3>
                    <button class="modal-close">&times;</button>
                </div>
                <div class="modal-body">
                    <p>This example demonstrates ${name.toLowerCase()} in Ovie.</p>
                    <p>Click the link below to view the complete example with explanations.</p>
                    <a href="${url}" class="btn btn-primary" target="_blank">View Full Example</a>
                </div>
            </div>
        `;
        
        document.body.appendChild(modal);
        
        // Close modal functionality
        const closeBtn = modal.querySelector('.modal-close');
        closeBtn.addEventListener('click', () => {
            document.body.removeChild(modal);
        });
        
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                document.body.removeChild(modal);
            }
        });
        
        // Auto-remove modal after 10 seconds
        setTimeout(() => {
            if (document.body.contains(modal)) {
                document.body.removeChild(modal);
            }
        }, 10000);
    }
}

// Touch interactions for iOS 26
function initializeTouchInteractions() {
    // Add touch feedback to buttons
    const buttons = document.querySelectorAll('.btn, .tab-btn, .example-btn, .mobile-menu-toggle');
    
    buttons.forEach(button => {
        button.addEventListener('touchstart', function() {
            this.style.transform = 'scale(0.95)';
            this.style.transition = 'transform 0.1s ease';
            
            // iOS 26 haptic feedback simulation
            if (navigator.vibrate) {
                navigator.vibrate(5);
            }
        });
        
        button.addEventListener('touchend', function() {
            this.style.transform = 'scale(1)';
            this.style.transition = 'transform 0.2s cubic-bezier(0.68, -0.55, 0.265, 1.55)';
        });
        
        button.addEventListener('touchcancel', function() {
            this.style.transform = 'scale(1)';
            this.style.transition = 'transform 0.2s ease';
        });
    });
    
    // Add swipe gestures for mobile navigation
    let touchStartX = 0;
    let touchStartY = 0;
    
    document.addEventListener('touchstart', function(e) {
        touchStartX = e.touches[0].clientX;
        touchStartY = e.touches[0].clientY;
    });
    
    document.addEventListener('touchend', function(e) {
        if (!touchStartX || !touchStartY) return;
        
        const touchEndX = e.changedTouches[0].clientX;
        const touchEndY = e.changedTouches[0].clientY;
        
        const deltaX = touchEndX - touchStartX;
        const deltaY = touchEndY - touchStartY;
        
        // Swipe right to open mobile menu (from left edge)
        if (deltaX > 100 && Math.abs(deltaY) < 100 && touchStartX < 50) {
            const mobileMenu = document.querySelector('.mobile-menu');
            if (mobileMenu && !mobileMenu.classList.contains('active')) {
                const mobileToggle = document.querySelector('.mobile-menu-toggle');
                if (mobileToggle) {
                    mobileToggle.click();
                }
            }
        }
        
        // Swipe left to close mobile menu
        if (deltaX < -100 && Math.abs(deltaY) < 100) {
            const mobileMenu = document.querySelector('.mobile-menu');
            if (mobileMenu && mobileMenu.classList.contains('active')) {
                const mobileToggle = document.querySelector('.mobile-menu-toggle');
                if (mobileToggle) {
                    mobileToggle.click();
                }
            }
        }
        
        touchStartX = 0;
        touchStartY = 0;
    });
}

// Downloads functionality
function initializeDownloads() {
    // Add download tracking
    const downloadLinks = document.querySelectorAll('a[href*=".exe"], a[href*="download"]');
    
    downloadLinks.forEach(link => {
        link.addEventListener('click', function() {
            // Track download analytics (simulated)
            console.log('Download started:', this.href);
            
            // Show download feedback
            const originalText = this.textContent;
            this.textContent = 'Downloading...';
            
            setTimeout(() => {
                this.textContent = originalText;
            }, 2000);
        });
    });
    
    // Add copy functionality to code blocks
    const codeBlocks = document.querySelectorAll('.code-block, .code-preview');
    
    codeBlocks.forEach(block => {
        // Create copy button if it doesn't exist
        if (!block.querySelector('.copy-btn')) {
            const copyBtn = document.createElement('button');
            copyBtn.className = 'copy-btn';
            copyBtn.innerHTML = '📋';
            copyBtn.title = 'Copy code';
            copyBtn.style.cssText = `
                position: absolute;
                top: 8px;
                right: 8px;
                background: rgba(255, 255, 255, 0.1);
                border: none;
                color: var(--text-primary);
                padding: 8px;
                border-radius: 6px;
                cursor: pointer;
                font-size: 14px;
                transition: all 0.2s ease;
                backdrop-filter: blur(10px);
            `;
            
            copyBtn.addEventListener('click', function() {
                copyCode(this);
            });
            
            copyBtn.addEventListener('mouseenter', function() {
                this.style.background = 'rgba(245, 158, 11, 0.2)';
            });
            
            copyBtn.addEventListener('mouseleave', function() {
                this.style.background = 'rgba(255, 255, 255, 0.1)';
            });
            
            block.style.position = 'relative';
            block.appendChild(copyBtn);
        }
    });
    
    // Enhanced copy functionality
    function copyCode(button) {
        const codeBlock = button.parentElement.querySelector('code, pre');
        if (!codeBlock) return;
        
        const text = codeBlock.textContent;
        
        navigator.clipboard.writeText(text).then(() => {
            const originalContent = button.innerHTML;
            button.innerHTML = '✅';
            button.style.background = 'rgba(35, 134, 54, 0.3)';
            
            setTimeout(() => {
                button.innerHTML = originalContent;
                button.style.background = 'rgba(255, 255, 255, 0.1)';
            }, 2000);
            
            // iOS 26 haptic feedback
            if (navigator.vibrate) {
                navigator.vibrate(10);
            }
        }).catch(err => {
            console.error('Failed to copy code:', err);
            button.innerHTML = '❌';
            button.style.background = 'rgba(218, 54, 51, 0.3)';
            
            setTimeout(() => {
                button.innerHTML = '📋';
                button.style.background = 'rgba(255, 255, 255, 0.1)';
            }, 2000);
        });
    }
}

// Utility functions
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

function throttle(func, limit) {
    let inThrottle;
    return function() {
        const args = arguments;
        const context = this;
        if (!inThrottle) {
            func.apply(context, args);
            inThrottle = true;
            setTimeout(() => inThrottle = false, limit);
        }
    }
}

// Copy code functionality
function copyCode(button) {
    const codeBlock = button.nextElementSibling.querySelector('code');
    const text = codeBlock.textContent;
    
    navigator.clipboard.writeText(text).then(() => {
        const originalText = button.textContent;
        button.textContent = 'Copied!';
        button.classList.add('copied');
        
        setTimeout(() => {
            button.textContent = originalText;
            button.classList.remove('copied');
        }, 2000);
    }).catch(err => {
        console.error('Failed to copy code:', err);
    });
}

// Newsletter subscription
function subscribeNewsletter(email) {
    // Simulate newsletter subscription
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            if (email && email.includes('@')) {
                resolve('Successfully subscribed!');
            } else {
                reject('Please enter a valid email address.');
            }
        }, 1000);
    });
}

// Performance monitoring
function trackPerformance() {
    if ('performance' in window) {
        window.addEventListener('load', () => {
            const perfData = performance.getEntriesByType('navigation')[0];
            console.log('Page load time:', perfData.loadEventEnd - perfData.loadEventStart, 'ms');
        });
    }
}

// Initialize performance tracking
trackPerformance();

// Add modern scroll effects
function initializeScrollEffects() {
    const navbar = document.querySelector('.navbar');
    let lastScrollY = window.scrollY;
    
    window.addEventListener('scroll', () => {
        const currentScrollY = window.scrollY;
        
        // Hide/show navbar on scroll
        if (currentScrollY > lastScrollY && currentScrollY > 100) {
            navbar.style.transform = 'translateY(-100%)';
        } else {
            navbar.style.transform = 'translateY(0)';
        }
        
        lastScrollY = currentScrollY;
    });
    
    // Parallax effect for hero section
    const hero = document.querySelector('.hero');
    if (hero) {
        window.addEventListener('scroll', () => {
            const scrolled = window.pageYOffset;
            const rate = scrolled * -0.5;
            hero.style.transform = `translateY(${rate}px)`;
        });
    }
}

// Initialize modern effects
document.addEventListener('DOMContentLoaded', function() {
    initializeScrollEffects();
    
    // Add loading states to buttons
    const buttons = document.querySelectorAll('.btn');
    buttons.forEach(button => {
        button.addEventListener('click', function() {
            if (this.href && !this.href.startsWith('#')) {
                this.classList.add('loading');
                setTimeout(() => {
                    this.classList.remove('loading');
                }, 2000);
            }
        });
    });
    
    // Add intersection observer for animations
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };
    
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('fade-in-up');
            }
        });
    }, observerOptions);
    
    // Observe all cards and sections
    const elementsToAnimate = document.querySelectorAll('.feature-card, .example-card, .library-module, .community-card, .doc-category');
    elementsToAnimate.forEach(el => observer.observe(el));
});

// Export functions for global access
window.OvieWebsite = {
    copyCode,
    subscribeNewsletter,
    debounce,
    throttle
};