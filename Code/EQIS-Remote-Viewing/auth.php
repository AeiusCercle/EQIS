<?php
// Set error logging
ini_set('display_errors', 1);
ini_set('display_startup_errors', 1);
error_reporting(E_ALL);

// Log errors to a file
ini_set('log_errors', 1);
ini_set('error_log', 'php_errors.log');

// Set headers to prevent caching
header("Cache-Control: no-store, no-cache, must-revalidate, max-age=0");
header("Cache-Control: post-check=0, pre-check=0", false);
header("Pragma: no-cache");

// Define authorized roles and their passwords
$auth_keys = [
    'human-monitor' => 'hm2025farsight',
    'ai-monitor' => 'am2025farsight'
];

// Initialize response array
$response = ['success' => false, 'message' => '', 'content' => ''];

// Check if role and key are provided
if (isset($_POST['role']) && isset($_POST['key']) && isset($_POST['target'])) {
    $role = $_POST['role'];
    $key = $_POST['key'];
    $target = $_POST['target'];
    
    // Debug log
    error_log("Auth attempt: Role=$role, Target=$target");
    
    // Validate role is authorized for target
    $valid_role = false;
    
    if ($target === 'ai-viewer-target' && $role === 'human-monitor') {
        $valid_role = true;
    } else if ($target === 'human-viewer-target' && $role === 'ai-monitor') {
        $valid_role = true;
    }
    
    // Validate credentials
    if ($valid_role && isset($auth_keys[$role]) && $auth_keys[$role] === $key) {
        // Authentication successful
        $response['success'] = true;
        
        // Retrieve the appropriate content file based on target
        $file_path = '';
        if ($target === 'ai-viewer-target') {
            $file_path = 'secure/ai_viewer_target.html';
        } else if ($target === 'human-viewer-target') {
            $file_path = 'secure/human_viewer_target.html';
        }
        
        error_log("Looking for file: $file_path");
        
        if (file_exists($file_path)) {
            $response['content'] = file_get_contents($file_path);
            $response['message'] = 'Access granted';
            error_log("File found and content loaded");
        } else {
            $response['success'] = false;
            $response['message'] = "Target content file not found: $file_path";
            error_log("File not found: $file_path");
            
            // List files in the secure directory for debugging
            $secure_files = scandir('secure');
            error_log("Files in secure directory: " . implode(", ", $secure_files));
        }
    } else {
        // Authentication failed
        $response['message'] = 'Invalid credentials or unauthorized role for this target';
        error_log("Auth failed: valid_role=$valid_role, role exists=" . (isset($auth_keys[$role]) ? 'true' : 'false') . ", password match=" . ($auth_keys[$role] === $key ? 'true' : 'false'));
    }
} else {
    $response['message'] = 'Missing required parameters';
    $missing = [];
    if (!isset($_POST['role'])) $missing[] = 'role';
    if (!isset($_POST['key'])) $missing[] = 'key';
    if (!isset($_POST['target'])) $missing[] = 'target';
    error_log("Missing parameters: " . implode(", ", $missing));
}

// Return JSON response
header('Content-Type: application/json');
echo json_encode($response);
?>
