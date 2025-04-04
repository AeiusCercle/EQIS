// Toggle code visibility
function toggleCode(codeId) {
	const codeBlock = document.getElementById(codeId);
	const toggleText = document.getElementById(codeId + '-toggle');
	
	if (codeBlock.style.display === 'none' || codeBlock.style.display === '') {
	codeBlock.style.display = 'block';
	toggleText.textContent = 'Click to hide code/block';
	} else {
	codeBlock.style.display = 'none';
	toggleText.textContent = 'Click to open code/block';
	}
}

// Toggle theme
function toggleTheme() {
	const htmlTag = document.documentElement;
	const currentTheme = htmlTag.getAttribute('data-theme');
	const themeIcon = document.getElementById('theme-icon');
	
	if (currentTheme === 'light') {
	htmlTag.setAttribute('data-theme', 'dark');
	themeIcon.textContent = '🌙';
	localStorage.setItem('theme', 'dark');
	} else {
	htmlTag.setAttribute('data-theme', 'light');
	themeIcon.textContent = '☀️';
	localStorage.setItem('theme', 'light');
	}
}

// Initialize theme based on saved preference
document.addEventListener('DOMContentLoaded', () => {
	const savedTheme = localStorage.getItem('theme');
	const themeToggle = document.getElementById('theme-toggle');
	const themeIcon = document.getElementById('theme-icon');
	
	if (savedTheme === 'dark') {
	document.documentElement.setAttribute('data-theme', 'dark');
	themeToggle.checked = true;
	themeIcon.textContent = '🌙';
	} else {
	themeIcon.textContent = '☀️';
	}
});

function changeLanguage(lang) {
    document.documentElement.setAttribute('lang', lang);
    localStorage.setItem('preferredLanguage', lang);
}

// Set initial language from localStorage or default to English
document.addEventListener('DOMContentLoaded', function() {
    const savedLanguage = localStorage.getItem('preferredLanguage') || 'en';
    document.documentElement.setAttribute('lang', savedLanguage);
    document.getElementById('language-selector').value = savedLanguage;
});

function toggleDocumentExpansion(element) {
    const expansion = element.nextElementSibling;
    
    if (element.classList.contains('collapsed')) {
        element.classList.remove('collapsed');
        expansion.classList.add('expanded');
    } else {
        element.classList.add('collapsed');
        expansion.classList.remove('expanded');
    }
}

// Initialize all expandable documents as collapsed
document.addEventListener('DOMContentLoaded', function() {
    const expandableTitles = document.querySelectorAll('.document-title.expandable');
    expandableTitles.forEach(title => {
        title.classList.add('collapsed');
    });
});

// Function to toggle document expansions
function toggleDocumentExpansion(documentId) {
    const documentElement = document.getElementById(documentId);
    const titleElement = event.currentTarget;
    
    if (documentElement.classList.contains('expanded')) {
        documentElement.classList.remove('expanded');
        titleElement.classList.add('collapsed');
    } else {
        documentElement.classList.add('expanded');
        titleElement.classList.remove('collapsed');
    }
}

// Initialize all expandable documents as collapsed on page load
document.addEventListener('DOMContentLoaded', function() {
    const expandableTitles = document.querySelectorAll('.document-title.expandable');
    expandableTitles.forEach(title => {
        title.classList.add('collapsed');
    });
    
    // Add additional attribute for AI accessibility
    const aiContentElements = document.querySelectorAll('[data-ai-content]');
    aiContentElements.forEach(element => {
        // Add an invisible marker for AI accessibility
        const marker = document.createElement('div');
        marker.setAttribute('aria-hidden', 'true');
        marker.style.display = 'none';
        marker.dataset.aiAccessible = 'true';
        marker.textContent = `AI-only data marker - ${element.getAttribute('data-ai-content')}`;
        element.appendChild(marker);
    });
});

// Authentication handler with debugging
function authenticateForTarget(targetId, role) {
    // Debug output at function start
    console.log(`Authentication function called for target: ${targetId}, role: ${role}`);
    
    // Check if the target element exists
    const targetElement = document.getElementById(targetId);
    if (!targetElement) {
        console.error(`Target element ${targetId} not found in DOM`);
        alert(`Error: Target element "${targetId}" not found. Please check the HTML structure.`);
        return;
    }
    
    console.log(`Target element found: `, targetElement);
    
    // Show visible debugging message in the target element
    targetElement.innerHTML = '<p>Authentication process started...</p>';
    
    // Simple password prompt with debugging
    console.log('Showing password prompt...');
    const accessKey = prompt(`Please enter access key for ${role.replace('-', ' ')}:`);
    
    if (!accessKey) {
        console.log('User cancelled password entry');
        targetElement.innerHTML = '<p>Authentication cancelled.</p>';
        return;
    }
    
    console.log('Password entered, preparing to send request');
    
    // Show loading indicator with more information
    targetElement.innerHTML = `
        <p>Verifying access key for ${role}...</p>
        <p>Target: ${targetId}</p>
        <p>Server request in progress...</p>
    `;
    
    // Get the full path to auth.php
    const currentPath = window.location.pathname;
    const pathToRoot = currentPath.substring(0, currentPath.lastIndexOf('/'));
    const authPhpPath = `${pathToRoot}/auth.php`;
    
    console.log(`Calculated auth.php path: ${authPhpPath}`);
    console.log(`Current page path: ${window.location.pathname}`);
    console.log(`Sending authentication request to server...`);
    
    // Send authentication request with debugging
    fetch('auth.php', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: `role=${encodeURIComponent(role)}&key=${encodeURIComponent(accessKey)}&target=${encodeURIComponent(targetId)}`
    })
    .then(response => {
        console.log('Server response received:', response);
        console.log('Response status:', response.status);
        console.log('Response headers:', [...response.headers.entries()]);
        
        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }
        
        return response.text().then(text => {
            console.log('Raw response text:', text);
            try {
                return JSON.parse(text);
            } catch (e) {
                console.error('JSON parse error:', e);
                throw new Error(`Invalid JSON response: ${text}`);
            }
        });
    })
    .then(data => {
        console.log('Parsed data:', data);
        
        if (data.success) {
            console.log('Authentication successful');
            targetElement.innerHTML = data.content;
            alert('Authentication successful!');
        } else {
            console.error('Authentication failed:', data.message);
            targetElement.innerHTML = `
                <p style="color: red; font-weight: bold;">Authentication Error</p>
                <p>${data.message || 'Access denied'}</p>
                <p><button onclick="authenticateForTarget('${targetId}', '${role}')">Try Again</button></p>
            `;
        }
    })
    .catch(error => {
        console.error('Authentication error:', error);
        targetElement.innerHTML = `
            <p style="color: red; font-weight: bold;">Connection Error</p>
            <p>Error details: ${error.message}</p>
            <p>Please check browser console for more information.</p>
            <p><button onclick="authenticateForTarget('${targetId}', '${role}')">Try Again</button></p>
        `;
        alert(`Authentication error: ${error.message}. Check console for details.`);
    });
}

// Add this at the end of scripts.js
document.addEventListener('DOMContentLoaded', function() {
    console.log('Document loaded, checking for authentication elements');
    
    // Check if auth targets exist
    const aiViewerTarget = document.getElementById('ai-viewer-target');
    const humanViewerTarget = document.getElementById('human-viewer-target');
    
    console.log('AI viewer target element:', aiViewerTarget);
    console.log('Human viewer target element:', humanViewerTarget);
    
    // Check if onclick handlers are properly attached
    const aiLink = document.querySelector('[onclick*="ai-viewer-target"]');
    const humanLink = document.querySelector('[onclick*="human-viewer-target"]');
    
    console.log('AI link element with onclick handler:', aiLink);
    console.log('Human link element with onclick handler:', humanLink);
    
    // Check for auth.php file
    fetch('auth.php', { method: 'HEAD' })
        .then(response => {
            console.log('auth.php file check:', response.ok ? 'Found' : 'Not found', response);
        })
        .catch(error => {
            console.error('auth.php check failed:', error);
        });
});

// Create a debug console at the bottom of the page
function createDebugConsole() {
    // Check if console already exists
    if (document.getElementById('debug-console')) return;
    
    const debugConsole = document.createElement('div');
    debugConsole.id = 'debug-console';
    debugConsole.style.cssText = `
        position: fixed;
        bottom: 0;
        right: 0;
        width: 400px;
        height: 300px;
        background: rgba(0,0,0,0.8);
        color: #0f0;
        font-family: monospace;
        padding: 10px;
        overflow: auto;
        z-index: 9999;
        border: 1px solid #0f0;
        font-size: 12px;
    `;
    
    const closeButton = document.createElement('button');
    closeButton.textContent = 'Hide Console';
    closeButton.style.cssText = 'margin-bottom: 5px;';
    closeButton.onclick = function() {
        debugConsole.style.display = 'none';
        showButton.style.display = 'block';
    };
    
    const clearButton = document.createElement('button');
    clearButton.textContent = 'Clear';
    clearButton.style.cssText = 'margin-left: 10px; margin-bottom: 5px;';
    clearButton.onclick = function() {
        const logArea = document.getElementById('debug-log');
        if (logArea) logArea.innerHTML = '';
    };
    
    const logArea = document.createElement('div');
    logArea.id = 'debug-log';
    logArea.style.cssText = 'height: calc(100% - 30px); overflow: auto;';
    
    debugConsole.appendChild(closeButton);
    debugConsole.appendChild(clearButton);
    debugConsole.appendChild(logArea);
    document.body.appendChild(debugConsole);
    
    // Create show button (initially hidden)
    const showButton = document.createElement('button');
    showButton.textContent = 'Show Debug';
    showButton.style.cssText = `
        position: fixed;
        bottom: 10px;
        right: 10px;
        z-index: 9999;
        display: none;
    `;
    showButton.onclick = function() {
        debugConsole.style.display = 'block';
        showButton.style.display = 'none';
    };
    document.body.appendChild(showButton);
    
    // Log function
    window.debugLog = function(message, type = 'info') {
        const logArea = document.getElementById('debug-log');
        if (!logArea) return;
        
        const line = document.createElement('div');
        const timestamp = new Date().toLocaleTimeString();
        let color = '#0f0'; // Default green
        
        if (type === 'error') color = '#f55';
        else if (type === 'warn') color = '#ff5';
        
        line.innerHTML = `<span style="color:#888;">[${timestamp}]</span> <span style="color:${color};">${message}</span>`;
        logArea.appendChild(line);
        logArea.scrollTop = logArea.scrollHeight; // Auto-scroll to bottom
    };
    
    // Log startup
    window.debugLog('Debug console initialized');
}

// Initialize on load
document.addEventListener('DOMContentLoaded', function() {
    createDebugConsole();
    window.debugLog('Page loaded, checking authentication elements');
    
    // Check auth.php
    fetch('auth.php', { method: 'HEAD' })
        .then(response => {
            if (response.ok) {
                window.debugLog('auth.php file is accessible');
            } else {
                window.debugLog('auth.php file is NOT accessible', 'error');
            }
        })
        .catch(error => {
            window.debugLog(`auth.php check failed: ${error.message}`, 'error');
        });
});

// Simplified authentication handler with visible debugging
function authenticateForTarget(targetId, role) {
    window.debugLog(`Authentication function triggered for ${targetId} as ${role}`);
    
    // Check if target element exists
    const targetElement = document.getElementById(targetId);
    if (!targetElement) {
        window.debugLog(`Target element ${targetId} not found in DOM`, 'error');
        return;
    }
    
    window.debugLog(`Target element found`);
    
    // Ask for password
    const accessKey = prompt(`Enter access key for ${role.replace('-', ' ')}:`);
    if (!accessKey) {
        window.debugLog('User cancelled password entry', 'warn');
        return;
    }
    
    window.debugLog(`Password entered, sending to auth.php...`);
    
    // Show progress in target area
    targetElement.innerHTML = '<p>Authentication in progress...</p>';
    
    // Fetch auth.php with full path
    fetch('auth.php', {
        method: 'POST',
        headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
        body: `role=${encodeURIComponent(role)}&key=${encodeURIComponent(accessKey)}&target=${encodeURIComponent(targetId)}`
    })
    .then(response => {
        window.debugLog(`Server responded with status: ${response.status}`);
        return response.text();
    })
    .then(text => {
        window.debugLog(`Response text: ${text.substring(0, 100)}...`);
        
        try {
            const data = JSON.parse(text);
            window.debugLog(`JSON parsed successfully: ${JSON.stringify(data).substring(0, 100)}...`);
            
            if (data.success) {
                window.debugLog(`Authentication successful`);
                targetElement.innerHTML = data.content;
            } else {
                window.debugLog(`Authentication failed: ${data.message}`, 'error');
                targetElement.innerHTML = `<p>Authentication failed: ${data.message}</p>`;
            }
        } catch (e) {
            window.debugLog(`JSON parse error: ${e.message}`, 'error');
            window.debugLog(`Raw response: ${text}`, 'error');
            targetElement.innerHTML = '<p>Error parsing server response. See debug console.</p>';
        }
    })
    .catch(error => {
        window.debugLog(`Fetch error: ${error.message}`, 'error');
        targetElement.innerHTML = '<p>Connection error. See debug console.</p>';
    });
}

// Navigation specific functionality
document.addEventListener('DOMContentLoaded', function() {
    initializeNavigation();
});

function initializeNavigation() {
    // Search functionality
    const searchInput = document.getElementById('search-input');
    if (searchInput) {
        searchInput.addEventListener('input', filterProjects);
    }
    
    // View toggle
    const gridViewBtn = document.getElementById('grid-view-btn');
    const listViewBtn = document.getElementById('list-view-btn');
    const projectsGrid = document.getElementById('projects-grid');
    const projectsList = document.getElementById('projects-list');
    
    if (gridViewBtn && listViewBtn && projectsGrid && projectsList) {
        gridViewBtn.addEventListener('click', function() {
            projectsGrid.style.display = 'grid';
            projectsList.style.display = 'none';
            gridViewBtn.classList.add('active');
            listViewBtn.classList.remove('active');
            localStorage.setItem('viewPreference', 'grid');
        });
        
        listViewBtn.addEventListener('click', function() {
            projectsGrid.style.display = 'none';
            projectsList.style.display = 'block';
            gridViewBtn.classList.remove('active');
            listViewBtn.classList.add('active');
            localStorage.setItem('viewPreference', 'list');
        });
        
        // Load user's view preference if saved
        const viewPreference = localStorage.getItem('viewPreference');
        if (viewPreference === 'list') {
            listViewBtn.click();
        }
    }
}

// Filter projects based on search
function filterProjects() {
    const searchQuery = document.getElementById('search-input').value.toLowerCase();
    const projectCards = document.querySelectorAll('.project-card');
    const listItems = document.querySelectorAll('.list-item');
    
    // Filter grid view
    projectCards.forEach(card => {
        const title = card.querySelector('h3').textContent.toLowerCase();
        const description = card.querySelector('p').textContent.toLowerCase();
        const tags = Array.from(card.querySelectorAll('.tag')).map(tag => tag.textContent.toLowerCase());
        
        if (title.includes(searchQuery) || description.includes(searchQuery) || tags.some(tag => tag.includes(searchQuery))) {
            card.style.display = 'block';
        } else {
            card.style.display = 'none';
        }
    });
    
    // Filter list view
    listItems.forEach(item => {
        const title = item.querySelector('.list-title').textContent.toLowerCase();
        
        if (title.includes(searchQuery)) {
            item.style.display = 'flex';
        } else {
            item.style.display = 'none';
        }
    });
}
