document.addEventListener("DOMContentLoaded", function() {
    const sidebarTrigger = document.querySelector(".sidebar-trigger");
    const sidebar = document.querySelector(".sidebar");
  
    // Function to toggle sidebar
    function toggleSidebar() {
      sidebar.classList.toggle("active");
    }
  
    // Event listener for sidebar trigger click
    sidebarTrigger.addEventListener("click", toggleSidebar);
  });
  