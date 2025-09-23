#!/bin/bash

# Function to generate dynamic commit message
generate_commit_message() {
    # Get list of modified/added files
    local modified_files=$(git diff --cached --name-only)
    local new_files=$(git diff --cached --name-status | grep "^A" | cut -f2)
    local changed_files=$(git diff --cached --name-status | grep "^M" | cut -f2)
    local deleted_files=$(git diff --cached --name-status | grep "^D" | cut -f2)
    
    local commit_msg=""
    
    # Count changes
    local added_count=$(echo "$new_files" | wc -l | tr -d ' ')
    local modified_count=$(echo "$changed_files" | wc -l | tr -d ' ')
    local deleted_count=$(echo "$deleted_files" | wc -l | tr -d ' ')
    
    # Remove empty lines from counts
    if [ -z "$new_files" ]; then added_count=0; fi
    if [ -z "$changed_files" ]; then modified_count=0; fi
    if [ -z "$deleted_files" ]; then deleted_count=0; fi
    
    # Detect exercise/project names from modified files
    local exercises=$(echo "$modified_files" | grep -E "^[^/]+/(src|tests)/" | cut -d'/' -f1 | sort -u | head -3)
    
    if [ ! -z "$exercises" ]; then
        local exercise_list=$(echo "$exercises" | tr '\n' ', ' | sed 's/,$//')
        if [ $(echo "$exercises" | wc -l) -gt 1 ]; then
            commit_msg="Update multiple exercises: $exercise_list"
        else
            commit_msg="Update $exercise_list exercise"
        fi
    else
        # Fallback to generic messages based on file types
        if [ $added_count -gt 0 ] && [ $modified_count -eq 0 ]; then
            commit_msg="Add new files"
        elif [ $added_count -eq 0 ] && [ $modified_count -gt 0 ]; then
            commit_msg="Update existing files"
        elif [ $deleted_count -gt 0 ]; then
            commit_msg="Remove files and update project"
        else
            commit_msg="Update project files"
        fi
    fi
    
    # Add file count details
    local details=""
    if [ $added_count -gt 0 ]; then
        details="${details}+${added_count}"
    fi
    if [ $modified_count -gt 0 ]; then
        if [ ! -z "$details" ]; then details="${details} "; fi
        details="${details}~${modified_count}"
    fi
    if [ $deleted_count -gt 0 ]; then
        if [ ! -z "$details" ]; then details="${details} "; fi
        details="${details}-${deleted_count}"
    fi
    
    if [ ! -z "$details" ]; then
        commit_msg="${commit_msg} (${details})"
    fi
    
    echo "$commit_msg"
}

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    exit 1
fi

# Add all changes
git add .

# Check if there are any changes to commit
if git diff --cached --quiet; then
    exit 0
fi

# Generate dynamic commit message
COMMIT_MSG=$(generate_commit_message)

# Commit with dynamic message
git commit -m "$COMMIT_MSG" > /dev/null 2>&1

# Push to current branch
git push > /dev/null 2>&1