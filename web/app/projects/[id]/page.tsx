'use client'

import React, {useEffect, useState} from 'react';
import {Project} from "@/app/api/models/project";
import {ProjectState} from "@/app/api/models/project-state";
import {useParams} from "next/navigation";
import NavBar from "@/app/components/NavBar";
import {ProjectGroup} from "@/app/api/models/project-group";
import GroupComponent from "@/app/components/data/GroupComponent";
import {showModal} from "@/app/utils";
import {FaPlus} from "react-icons/fa";
import NewGroupModal from "@/app/components/modals/NewGroupModal";

async function getProject(project_id: string): Promise<Project> {
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${api_url}/projects/${project_id}`, {
        headers: { "Content-Type": "application/json" },
        credentials: "include",
    });
    const data = await response.json();
    return data as Project;
}

async function getGroupsFromProject(project_id: string): Promise<ProjectGroup[]> {
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${api_url}/groups/project/${project_id}`, {
        headers: { "Content-Type": "application/json" },
        credentials: "include",
    });
    const data = await response.json();
    return data.groups as ProjectGroup[];
}

const ProjectDetails = ({ project }: { project: Project }) => {
    return (
        <div className="p-4 bg-white rounded-lg shadow-md">
            <h2 className="text-xl font-semibold text-gray-800 mb-2">{project.name}</h2>
            <p className="text-gray-600">{project.description}</p>
            <p className="text-gray-600"><strong>Start Date:</strong> {project.start_date}</p>
            <p className="text-gray-600"><strong>End Date:</strong> {project.end_date}</p>
            <p className="text-gray-600"><strong>Duration:</strong> {project.notation_period_duration} days</p>
            <p className={`text-sm rounded-full px-2 py-1 ${project.state === ProjectState.Finished ? 'bg-green-200 text-green-800' : 'bg-yellow-200 text-yellow-800'}`}>
                {project.state}
            </p>
        </div>
    );
};

const ProjectDetailsComponent: React.FC<{ project_id: string; project: Project; setProject: React.Dispatch<React.SetStateAction<Project>>; }> = ({ project_id, project, setProject }) => {
    const [projectLoading, setProjectLoading] = useState(true);
    const [projectError, setProjectError] = useState<string | null>(null);

    useEffect(() => {
        const fetchProject = async () => {
            try {
                setProjectLoading(true);
                const projectData = await getProject(project_id);
                setProject(projectData);
            } catch (err) {
                setProjectError('Error fetching project details.');
            } finally {
                setProjectLoading(false);
            }
        };
        fetchProject();
    }, [project_id]);

    return (
        <div className="my-4">
            {projectLoading ? (
                <p className="text-gray-600">Loading project details...</p>
            ) : projectError ? (
                <p className="text-red-600">{projectError}</p>
            ) : (
                <ProjectDetails project={project} />
            )}
        </div>
    );
};

const ProjectPage: React.FC = () => {
    const { id: project_id } = useParams<{ id: string }>();

    const [groups, setGroups] = useState<ProjectGroup[]>([]);
    const [groupsLoading, setGroupsLoading] = useState(true);
    const [groupsError, setGroupsError] = useState<string | null>(null);

    const [project, setProject] = useState<Project>({
        id: '',
        description: '',
        end_date: "",
        name: "",
        notation_period_duration: 0,
        promotion_id: "",
        start_date: "",
        state: ProjectState.NotStarted
    });

    useEffect(() => {
        const fetchGroups = async () => {
            try {
                setGroupsLoading(true);
                const groupsData = await getGroupsFromProject(project_id);
                setGroups(groupsData);
            } catch (err) {
                setGroupsError('Error fetching groups.');
            } finally {
                setGroupsLoading(false);
            }
        };

        fetchGroups();
    }, [project_id]);

    return (
        <div className="min-h-screen bg-gray-200">
            <NavBar />
            <div className="container mx-auto py-8">
                <ProjectDetailsComponent project_id={project_id} project={project} setProject={setProject} />
                <div className="my-8">
                    {groupsLoading ? (
                        <p className="text-gray-600">Loading groups...</p>
                    ) : groupsError ? (
                        <p className="text-red-600">{groupsError}</p>
                    ) : groups && groups.length > 0 ? (
                        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                            {groups.map((group) => (
                                <GroupComponent
                                    key={group.group.id}
                                    info={group}
                                    withMarks={true}
                                    onDelete={(id: string) => {}}
                                    projectState={project.state}
                                />
                            ))}
                        </div>
                    ) : (
                        <p className="text-gray-600">No groups available.</p>
                    )}
                    <button onClick={() => showModal("new_group_modal")}
                            className="flex items-center justify-center w-10 h-10 bg-blue-600 text-white rounded-full hover:bg-blue-700 transition">
                        <FaPlus />
                    </button>
                </div>
            </div>
            <NewGroupModal groups={groups} setGroups={setGroups} project_id={project_id} key={project_id} />
        </div>
    );
};

export default ProjectPage;
